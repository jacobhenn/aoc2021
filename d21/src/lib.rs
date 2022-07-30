use std::{collections::HashMap, io, str::FromStr};

use derive_more::Display;

use anyhow::{Context, Result};

/// `ROLL_DIST[i]` = the number of universes in which the sum of three consecutive Dirac Dice rolls results in `i+3`.
const BRANCH_DISTRIBUTION: &[u64; 7] = &[1, 3, 6, 7, 6, 3, 1];

#[derive(Display, PartialEq, Eq, Clone, Copy, Hash, Debug, PartialOrd, Ord)]
#[display(fmt = "pos: {}, score: {}", pos, score)]
pub struct PlayerState {
    /// position on the board from 0 to 9
    pub pos: u8,
    pub score: u8,
}

impl FromStr for PlayerState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, pos) = s.split_once(": ").context("expected `: `")?;
        Ok(PlayerState {
            pos: pos.parse::<u8>().context("couldn't parse player position")? - 1,
            score: 0,
        })
    }
}

impl PlayerState {
    pub fn at(pos: u8) -> Self {
        Self { pos, score: 0 }
    }

    pub fn branch(self) -> Branches {
        Branches::new(self)
    }
}

pub struct Branches {
    orig_player: PlayerState,
    roll: u8,
}

impl Branches {
    fn new(orig_player: PlayerState) -> Self {
        Self {
            orig_player,
            roll: 3,
        }
    }
}

impl Iterator for Branches {
    type Item = PlayerState;

    fn next(&mut self) -> Option<Self::Item> {
        // the largest one can roll with three Dirac Dice is 9.
        if self.roll > 9 {
            return None;
        }

        let pos = (self.orig_player.pos + self.roll) % 10;
        let res = PlayerState {
            pos,
            score: self.orig_player.score + pos + 1,
        };

        self.roll += 1;
        Some(res)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, PartialOrd, Ord)]
pub struct GameState {
    player1: PlayerState,
    player2: PlayerState,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Player {
    P1,
    P2,
}

impl GameState {
    pub fn new(p1_pos: u8, p2_pos: u8) -> Self {
        Self {
            player1: PlayerState::at(p1_pos),
            player2: PlayerState::at(p2_pos),
        }
    }

    pub fn get(&self, player: Player) -> &PlayerState {
        match player {
            Player::P1 => &self.player1,
            Player::P2 => &self.player2,
        }
    }

    pub fn with_player_state(self, player: Player, state: PlayerState) -> Self {
        match player {
            Player::P1 => Self {
                player1: state,
                ..self
            },
            Player::P2 => Self {
                player2: state,
                ..self
            },
        }
    }
}

#[derive(Debug)]
pub struct Multiverse {
    /// a map of game states to the number of universes in which the game is currently in that state.
    pub state_counts: HashMap<GameState, u64>,
}

impl Multiverse {
    pub fn new(player1: PlayerState, player2: PlayerState) -> Self {
        let mut state_counts = HashMap::new();
        let initial_state = GameState { player1, player2 };

        state_counts.insert(initial_state, 1);

        Self { state_counts }
    }

    // propagate the given player's waveform. return the number of universes in which that player won *this turn*.
    pub fn tick_player(&mut self, player: Player) -> u64 {
        let mut new_state_counts = HashMap::new();
        let mut win_count = 0;
        for (game_state, count) in &self.state_counts {
            for (idx, branch) in game_state.get(player).branch().enumerate() {
                // some branches happen more frequently than others. multiply the original count by the "probability" of this particular branch happening so that we can store all of those overlapping branches in the same spot while still counting all of them.
                let new_count = count * BRANCH_DISTRIBUTION[idx];

                // we don't need to store universes in which `player` won. count them and discard them.
                if branch.score >= 21 {
                    win_count += new_count;
                    continue;
                }

                *new_state_counts
                    .entry(game_state.with_player_state(player, branch))
                    .or_insert(0) += new_count;
            }
        }

        self.state_counts = new_state_counts;
        win_count
    }

    /// perform two passes of the multiverse: one to advance player 1, and one to advance player 2. return the number of universes in which player 1 won *this turn*.
    pub fn tick_both(&mut self) -> u64 {
        let p1_win_count = self.tick_player(Player::P1);

        // this will also remove universes in which player 2 won first, so that we don't have to worry about them.
        self.tick_player(Player::P2);

        p1_win_count
    }
}

pub fn get_input() -> Result<Multiverse> {
    let mut lines = io::stdin().lines();
    let p1 = lines
        .next()
        .context("input ended unexpectedly")??
        .parse()
        .context("couldn't parse player starting pos")?;
    let p2 = lines
        .next()
        .context("input ended unexpectedly")??
        .parse()
        .context("couldn't parse player starting pos")?;
    Ok(Multiverse::new(p1, p2))
}
