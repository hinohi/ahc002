use std::collections::{BinaryHeap, HashSet};

use proconio::input;

const L: usize = 50;
const BEAM_WIDTH: usize = 4_000;

fn main() {
    input! {
        si: usize,
        sj: usize,
        tt: [u32; L * L],
        pp: [u32; L * L],
    }
    println!("{}", calc((si, sj), &tt, &pp));
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

fn calc(s: (usize, usize), tt: &[u32], pp: &[u32]) -> String {
    let mut visited = HashSet::new();
    visited.insert(tt[to_pos(s)]);
    let mut states = vec![State {
        score: pp[to_pos(s)],
        pos: s,
        log: String::new(),
        visited,
    }];
    let mut best = String::new();
    let mut best_score = 0;
    loop {
        let mut queue = BinaryHeap::with_capacity(BEAM_WIDTH + 1);
        for state in states {
            if state.score > best_score {
                best = state.log.clone();
                best_score = state.score;
            }
            if let Some(s) = move_up(&state, tt, pp) {
                queue.push(s);
                if queue.len() > BEAM_WIDTH {
                    queue.pop();
                }
            }
            if let Some(s) = move_down(&state, tt, pp) {
                queue.push(s);
                if queue.len() > BEAM_WIDTH {
                    queue.pop();
                }
            }
            if let Some(s) = move_left(&state, tt, pp) {
                queue.push(s);
                if queue.len() > BEAM_WIDTH {
                    queue.pop();
                }
            }
            if let Some(s) = move_right(&state, tt, pp) {
                queue.push(s);
                if queue.len() > BEAM_WIDTH {
                    queue.pop();
                }
            }
        }
        if queue.is_empty() {
            break;
        }
        states = queue.into_vec();
    }
    best
}

fn move_up(state: &State, tt: &[u32], pp: &[u32]) -> Option<State> {
    if state.pos.0 == 0 {
        return None;
    }
    let pos = (state.pos.0 - 1, state.pos.1);
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

fn move_down(state: &State, tt: &[u32], pp: &[u32]) -> Option<State> {
    if state.pos.0 + 1 == L {
        return None;
    }
    let pos = (state.pos.0 + 1, state.pos.1);
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

fn move_left(state: &State, tt: &[u32], pp: &[u32]) -> Option<State> {
    if state.pos.1 == 0 {
        return None;
    }
    let pos = (state.pos.0, state.pos.1 - 1);
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

fn move_right(state: &State, tt: &[u32], pp: &[u32]) -> Option<State> {
    if state.pos.1 + 1 == L {
        return None;
    }
    let pos = (state.pos.0, state.pos.1 + 1);
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
