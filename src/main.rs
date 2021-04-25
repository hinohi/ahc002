use std::collections::{BinaryHeap, HashMap, HashSet};
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
    let ranges = make_ranges(&[0, 25, 50]);
    let pos = ranges
        .iter()
        .position(|(r0, r1)| r0.contains(&s.0) && r1.contains(&s.1))
        .unwrap();
    let (a_ranges, b_ranges) = match pos {
        0 => ([0, 2, 3, 1], [0, 1, 3, 2]),
        1 => ([1, 0, 2, 3], [1, 3, 2, 0]),
        2 => ([2, 3, 1, 0], [2, 0, 1, 3]),
        3 => ([3, 1, 0, 2], [3, 2, 0, 1]),
        _ => unreachable!(),
    };

    let mut visited = HashSet::new();
    visited.insert(tt[to_pos(s)]);
    let state = State {
        score: pp[to_pos(s)],
        pos: s,
        log: String::new(),
        visited,
    };
    let a_ranges = a_ranges
        .iter()
        .map(|&i| ranges[i].clone())
        .collect::<Vec<_>>();
    let b_ranges = b_ranges
        .iter()
        .map(|&i| ranges[i].clone())
        .collect::<Vec<_>>();
    let a = calc(state.clone(), &tt, &pp, &a_ranges);
    let b = calc(state.clone(), &tt, &pp, &b_ranges);
    eprintln!("{:?}", a);
    eprintln!("{:?}", b);
    if a.0 < b.0 {
        println!("{}", b.1);
    } else {
        println!("{}", a.1);
    }
}

fn calc(
    state: State,
    tt: &[u32],
    pp: &[u32],
    ranges: &[(Range<usize>, Range<usize>)],
) -> (u32, String) {
    let mut states = vec![state];
    for i in 0..ranges.len() - 1 {
        let next_stats = if ranges[i].0 == ranges[i + 1].0 {
            if ranges[i].1.end == ranges[i + 1].1.start {
                // 右移動
                calc_edge1(states.clone(), tt, pp, &ranges[i], ranges[i].1.end - 1)
            } else {
                // 左移動
                calc_edge1(states.clone(), tt, pp, &ranges[i], ranges[i].1.start)
            }
        } else {
            if ranges[i].0.end == ranges[i + 1].0.start {
                // 下移動
                calc_edge0(states.clone(), tt, pp, &ranges[i], ranges[i].0.end - 1)
            } else {
                // 上移動
                calc_edge0(states.clone(), tt, pp, &ranges[i], ranges[i].0.start)
            }
        };
        if next_stats.is_empty() {
            eprintln!("{:?}", ranges);
            eprintln!("{:?}", i);
            return calc_bulk(states, tt, pp);
        }
        states = next_stats;
    }
    calc_bulk(states, tt, pp)
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
    range: &(Range<usize>, Range<usize>),
    next_edge: usize,
) -> Vec<State> {
    let mut edge = HashMap::new();
    loop {
        let mut queue = BinaryHeap::with_capacity(BEAM_WIDTH + 1);
        for state in states {
            if state.pos.0 == next_edge {
                let s = edge.entry(state.pos.1).or_insert_with(|| state.clone());
                if s.score < state.score {
                    *s = state.clone();
                }
            }
            move_4(&mut queue, &state, tt, pp, &range);
        }
        if queue.is_empty() {
            break;
        }
        states = queue.into_vec();
    }
    edge.drain().map(|(_, s)| s).collect()
}

fn calc_edge1(
    mut states: Vec<State>,
    tt: &[u32],
    pp: &[u32],
    range: &(Range<usize>, Range<usize>),
    next_edge: usize,
) -> Vec<State> {
    let mut edge = HashMap::new();
    loop {
        let mut queue = BinaryHeap::with_capacity(BEAM_WIDTH + 1);
        for state in states {
            if state.pos.1 == next_edge {
                let s = edge.entry(state.pos.0).or_insert_with(|| state.clone());
                if s.score < state.score {
                    *s = state.clone();
                }
            }
            move_4(&mut queue, &state, tt, pp, &range);
        }
        if queue.is_empty() {
            break;
        }
        states = queue.into_vec();
    }
    edge.drain().map(|(_, s)| s).collect()
}

fn calc_bulk(mut states: Vec<State>, tt: &[u32], pp: &[u32]) -> (u32, String) {
    let mut best = String::new();
    let mut best_score = 0;
    loop {
        let mut queue = BinaryHeap::with_capacity(BEAM_WIDTH + 1);
        for state in states {
            if state.score > best_score {
                best_score = state.score;
                best = state.log.clone();
            }
            move_4(&mut queue, &state, tt, pp, &(0..L, 0..L));
        }
        if queue.is_empty() {
            break;
        }
        states = queue.into_vec();
    }
    (best_score, best)
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
