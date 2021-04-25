use std::collections::{BinaryHeap, HashSet};
use std::ops::Range;

use proconio::input;

const L: usize = 50;
const BEAM_WIDTH: usize = 1_000;

fn make_ranges(rr: &[usize]) -> Vec<(Range<usize>, Range<usize>)> {
    let mut ranges = Vec::new();
    for i in 0..rr.len() - 1 {
        for j in 0..rr.len() - 1 {
            ranges.push((rr[i]..rr[i + 1], rr[j]..rr[j + 1]));
        }
    }
    ranges
}

fn main() {
    input! {
        s: (usize, usize),
        tt: [u32; L * L],
        pp: [u32; L * L],
    }

    let ranges = make_ranges(&[0, 16, 34, 50]);
    eprintln!("{:?}", ranges);

    let mut visited = HashSet::new();
    visited.insert(tt[to_pos(s)]);
    let states = vec![State {
        score: pp[to_pos(s)],
        pos: s,
        log: String::new(),
        visited,
    }];
    println!(
        "{}",
        calc_edge0(states, &tt, &pp, ranges[0].clone(), 16)[0].log
    );
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    score: u32,
    pos: (usize, usize),
    log: String,
    visited: HashSet<u32>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 逆転させる
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[inline]
fn to_pos(pos: (usize, usize)) -> usize {
    pos.0 * L + pos.1
}

fn calc_edge0(
    mut states: Vec<State>,
    tt: &[u32],
    pp: &[u32],
    range: (Range<usize>, Range<usize>),
    next_edge: usize,
) -> Vec<State> {
    let mut edge = vec![None; 17];
    loop {
        let mut queue = BinaryHeap::with_capacity(BEAM_WIDTH + 1);
        for state in states {
            if state.pos.0 == next_edge {
                if edge[state.pos.1].is_none() {
                    edge[state.pos.1] = Some(state.clone());
                } else if matches!(edge[state.pos.1], Some(ref s) if s.score < state.score) {
                    edge[state.pos.1] = Some(state.clone());
                }
            }
            move_4(&mut queue, &state, tt, pp, &range);
        }
        if queue.is_empty() {
            break;
        }
        states = queue.into_vec();
    }
    edge.into_iter().filter_map(|s| s).collect()
}

fn calc_edge1(
    mut states: Vec<State>,
    tt: &[u32],
    pp: &[u32],
    range: (Range<usize>, Range<usize>),
    next_edge: usize,
) -> Vec<State> {
    let mut edge = vec![None; 17];
    loop {
        let mut queue = BinaryHeap::with_capacity(BEAM_WIDTH + 1);
        for state in states {
            if state.pos.1 == next_edge {
                if edge[state.pos.0].is_none() {
                    edge[state.pos.0] = Some(state.clone());
                } else if matches!(edge[state.pos.0], Some(ref s) if s.score < state.score) {
                    edge[state.pos.0] = Some(state.clone());
                }
            }
            move_4(&mut queue, &state, tt, pp, &range);
        }
        if queue.is_empty() {
            break;
        }
        states = queue.into_vec();
    }
    edge.into_iter().filter_map(|s| s).collect()
}

fn calc_bulk(
    mut states: Vec<State>,
    tt: &[u32],
    pp: &[u32],
    range: (Range<usize>, Range<usize>),
) -> String {
    let mut best = String::new();
    let mut best_score = 0;
    loop {
        let mut queue = BinaryHeap::with_capacity(BEAM_WIDTH + 1);
        for state in states {
            if state.score > best_score {
                best_score = state.score;
                best = state.log.clone();
            }
            move_4(&mut queue, &state, tt, pp, &range);
        }
        if queue.is_empty() {
            break;
        }
        states = queue.into_vec();
    }
    best
}

fn move_4(
    queue: &mut BinaryHeap<State>,
    state: &State,
    tt: &[u32],
    pp: &[u32],
    range: &(Range<usize>, Range<usize>),
) {
    if let Some(s) = move_up(state, tt, pp, &range.0) {
        queue.push(s);
        if queue.len() > BEAM_WIDTH {
            queue.pop();
        }
    }
    if let Some(s) = move_down(state, tt, pp, &range.0) {
        queue.push(s);
        if queue.len() > BEAM_WIDTH {
            queue.pop();
        }
    }
    if let Some(s) = move_left(state, tt, pp, &range.1) {
        queue.push(s);
        if queue.len() > BEAM_WIDTH {
            queue.pop();
        }
    }
    if let Some(s) = move_right(state, tt, pp, &range.1) {
        queue.push(s);
        if queue.len() > BEAM_WIDTH {
            queue.pop();
        }
    }
}

fn move_up(state: &State, tt: &[u32], pp: &[u32], range: &Range<usize>) -> Option<State> {
    if state.pos.0 == 0 {
        return None;
    }
    let pos = (state.pos.0 - 1, state.pos.1);
    if !range.contains(&pos.0) {
        return None;
    }
    let tile = tt[to_pos(pos)];
    if state.visited.contains(&tile) {
        return None;
    }
    let mut state = state.clone();
    state.score += pp[to_pos(pos)];
    state.pos = pos;
    state.log.push('U');
    state.visited.insert(tile);
    Some(state)
}

fn move_down(state: &State, tt: &[u32], pp: &[u32], range: &Range<usize>) -> Option<State> {
    let pos = (state.pos.0 + 1, state.pos.1);
    if !range.contains(&pos.0) {
        return None;
    }
    let tile = tt[to_pos(pos)];
    if state.visited.contains(&tile) {
        return None;
    }
    let mut state = state.clone();
    state.score += pp[to_pos(pos)];
    state.pos = pos;
    state.log.push('D');
    state.visited.insert(tile);
    Some(state)
}

fn move_left(state: &State, tt: &[u32], pp: &[u32], range: &Range<usize>) -> Option<State> {
    if state.pos.1 == 0 {
        return None;
    }
    let pos = (state.pos.0, state.pos.1 - 1);
    if !range.contains(&pos.1) {
        return None;
    }
    let tile = tt[to_pos(pos)];
    if state.visited.contains(&tile) {
        return None;
    }
    let mut state = state.clone();
    state.score += pp[to_pos(pos)];
    state.pos = pos;
    state.log.push('L');
    state.visited.insert(tile);
    Some(state)
}

fn move_right(state: &State, tt: &[u32], pp: &[u32], range: &Range<usize>) -> Option<State> {
    let pos = (state.pos.0, state.pos.1 + 1);
    if !range.contains(&pos.1) {
        return None;
    }
    let tile = tt[to_pos(pos)];
    if state.visited.contains(&tile) {
        return None;
    }
    let mut state = state.clone();
    state.score += pp[to_pos(pos)];
    state.pos = pos;
    state.log.push('R');
    state.visited.insert(tile);
    Some(state)
}
