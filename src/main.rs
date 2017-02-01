#![feature(associated_consts)]
#![feature(cfg_target_feature)]
#![feature(asm)]
#![feature(test)]
#![feature(field_init_shorthand)]

#![allow(dead_code)]  // TODO: Remove

extern crate rayon;
extern crate test;

mod square;
mod bitboard;
mod attacks;
mod board;

use rayon::prelude::*;

use board::Board;
use board::Move;
use attacks::Precomp;


fn perft_inner(board: &Board, depth: i8, precomp: &Precomp) -> usize {
    if depth < 1 {
        1
    } else {
        let mut moves: Vec<Move> = Vec::new();
        board.legal_moves(&mut moves, precomp);

        if depth == 1 {
            moves.len()
        } else {
            moves.iter().map(|m| {
                let mut child = board.clone();
                child.do_move(m);
                perft_inner(&child, depth - 1, precomp)
            }).sum()
        }
    }
}

fn perft(board: &Board, depth: i8, precomp: &Precomp) -> usize {
    if depth < 1 {
        1
    } else {
        let mut moves: Vec<Move> = Vec::new();
        board.legal_moves(&mut moves, precomp);

        moves.par_iter().map(|m| {
            let mut child = board.clone();
            child.do_move(m);
            let p = perft_inner(&child, depth - 1, precomp);
            println!("{} {}", m, p);
            p
        }).sum()
    }
}

fn main() {
    let precomp = attacks::Precomp::new();
    let board = Board::new();

    //board.do_move(&Move::from_uci("e2e4").unwrap());
    //board.do_move(&Move::from_uci("e7e5").unwrap());
    //board.do_move(&Move::from_uci("f1b5").unwrap());

    //assert_eq!(perft(&board, 1, &precomp), 20);
    //assert_eq!(perft(&board, 2, &precomp), 400);
    //assert_eq!(perft(&board, 3, &precomp), 8902);
    //assert_eq!(perft(&board, 4, &precomp), 197281);
    assert_eq!(perft(&board, 5, &precomp), 4865609);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_perft(b: &mut Bencher) {
        let precomp = attacks::Precomp::new();
        let board = Board::new();
        b.iter(|| assert_eq!(perft_inner(&board, 4, &precomp), 197281));
    }
}
