use crate::util::parse::ParseOps;
use Direction::{Left, Right};

pub enum Direction {
    Left(u32),
    Right(u32),
}

impl Direction {
    pub fn distance(&self) -> u32 {
        match *self {
            Left(dist) | Right(dist) => dist,
        }
    }
}

pub fn parse(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|l| {
            let dist = l.unsigned();
            match l.chars().take(1).next() {
                Some('L') => Left(dist),
                _ => Right(dist),
            }
        })
        .collect()
}

const MAX_POS: u32 = 100;
const INIT_POS: u32 = 50;

pub fn part1(rotations: &[Direction]) -> u32 {
    rotations
        .iter()
        .scan(INIT_POS, |pos, dir| {
            *pos = match dir {
                Left(dist) => (MAX_POS - ((MAX_POS - *pos) + dist) % MAX_POS) % MAX_POS,
                Right(dist) => (*pos + dist) % MAX_POS,
            };
            Some(*pos == 0 && dir.distance() > 0)
        })
        .filter(|rotated_to_zero| *rotated_to_zero)
        .count() as u32
}

pub fn part2(rotations: &[Direction]) -> u32 {
    rotations
        .iter()
        .scan(INIT_POS, |pos, dir| {
            Some(match dir {
                Left(dist) => {
                    let crossed_zero = (((MAX_POS - *pos) % MAX_POS) + dist) / MAX_POS;
                    *pos = (MAX_POS - ((MAX_POS - *pos) + dist) % MAX_POS) % MAX_POS;
                    crossed_zero
                }
                Right(dist) => {
                    let crossed_zero = (*pos + dist) / MAX_POS;
                    *pos = (*pos + dist) % MAX_POS;
                    crossed_zero
                }
            })
        })
        .sum()
}

#[test]
fn sample_input() {
    let input = "\
        L68\n\
        L30\n\
        R48\n\
        L5\n\
        R60\n\
        L55\n\
        L1\n\
        L99\n\
        R14\n\
        L82\
        ";
    let rotations = parse(input);
    assert_eq!(part1(&rotations), 3);
    assert_eq!(part2(&rotations), 6);
}

#[test]
fn overflow() {
    let input = "\
        L200\n\
        R50\
        ";
    let passw = parse(input);
    assert_eq!(part1(&passw), 1);
    assert_eq!(part2(&passw), 3);
}

#[test]
fn no_recount_zero_dist() {
    let input = "\
        L50\n\
        L0\n\
        R0\
        ";
    let passw = parse(input);
    assert_eq!(part1(&passw), 1);
    assert_eq!(part2(&passw), 1);
}
