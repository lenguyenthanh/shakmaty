// This file is part of the shakmaty library.
// Copyright (C) 2017-2022 Niklas Fiekas <niklas.fiekas@backscattering.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use shakmaty::{fen::Fen, perft, CastlingMode, Chess, FromSetup, Position, PositionError, uci::Uci};

pub fn divide<P: Position + Clone>(pos: &P, depth: u32) {
    let moves = pos.legal_moves();
    let mut sum: u64 = 0;
    let mut xs: Vec<String> = moves.iter()
        .map(|m| {
            let mut child = pos.clone();
            child.play_unchecked(m);
            let nodes = perft(&child, depth - 1);
            sum += nodes;
            format!("{} {}", Uci::from_standard(m), nodes)
        }).collect();
    xs.sort();
    let x = xs.join("\n");
    println!("{0}\n{1}", x, sum);
}

fn test_perft_file<P>(path: &str, node_limit: u64)
where
    P: Position + FromSetup + Default + Clone,
{
    let file = File::open(path).expect("failed to open test suite");
    let reader = BufReader::new(file);

    let mut pos = P::default();

    for line in reader.lines().map(|l| l.unwrap()) {
        println!("{}", line);

        let trimmed = line.trim();
        let mut slices = trimmed.splitn(2, ' ');

        match slices.next() {
            Some("epd") => {
                pos = slices
                    .next()
                    .expect("missing epd")
                    .parse::<Fen>()
                    .expect("invalid fen")
                    .into_position(CastlingMode::Chess960)
                    .or_else(PositionError::ignore_impossible_check)
                    .expect("illegal fen");
            }
            Some("perft") => {
                let mut params = slices.next().expect("missing perft params").splitn(2, ' ');

                let depth = params
                    .next()
                    .expect("missing perft depth")
                    .parse()
                    .expect("depth not an integer");

                let nodes = params
                    .next()
                    .expect("missing perft nodes")
                    .parse()
                    .expect("nodes not an integer");

                if nodes <= node_limit {
                    divide(&pos, depth);
                    assert_eq!(perft(&pos, depth), nodes);
                }
            }
            _ => {}
        }
    }
}

// macro for generating tests
macro_rules! gen_tests {
    ($($fn_name:ident, $t:ty, $path:tt, $num:expr,)+) => {
        $(
            #[test]
            #[cfg_attr(miri, ignore)]
            fn $fn_name() {
                test_perft_file::<$t>($path, $num);
            }
        )+
    }
}

// gen_tests! {
//     test_tricky,      Chess,       "tests/tricky.perft",        100_000,
//     test_random,      Chess,       "tests/random.perft",         10_000,
//     test_chess960,    Chess,       "tests/chess960.perft",      100_000,
// }

#[cfg(feature = "variant")]
use shakmaty::variant::{Antichess, Atomic, Crazyhouse, Horde, RacingKings, ThreeCheck};
#[cfg(feature = "variant")]
gen_tests! {
    // test_atomic,      Atomic,      "tests/atomic.perft",      10_000_000,
    // test_antichess,   Antichess,   "tests/antichess.perft",   1_000_000,
    test_crazyhouse,  Crazyhouse,  "tests/crazyhouse.perft",  1_000_000,
    // test_racingkings, RacingKings, "tests/racingkings.perft", 1_000_000,
    // test_horde,       Horde,       "tests/horde.perft",       1_000_000,
    // test_3check,      ThreeCheck,  "tests/3check.perft",      1_000_000,
}
