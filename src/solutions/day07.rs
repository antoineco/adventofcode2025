use std::collections::HashSet;

pub fn parse(input: &str) -> (usize, Vec<Vec<usize>>) {
    let mut iter_lines = input.lines();
    let beam_start = iter_lines.next().unwrap().find('S').unwrap();
    let splitters = iter_lines
        .map(|l| l.match_indices('^').map(|(i, _)| i).collect())
        .collect();

    (beam_start, splitters)
}

pub fn part1(manifold: &(usize, Vec<Vec<usize>>)) -> usize {
    let (beam_start, splitters) = manifold;

    splitters
        .iter()
        .scan(HashSet::from([*beam_start]), |cur_beams, lvl| {
            let new_splits = lvl.iter().fold(0, |acc, s| {
                acc + if cur_beams.contains(s) {
                    cur_beams.take(s);
                    cur_beams.insert(s - 1);
                    cur_beams.insert(s + 1);
                    1
                } else {
                    0
                }
            });
            Some(new_splits)
        })
        .sum()
}

pub fn part2(_: &(usize, Vec<Vec<usize>>)) -> usize {
    0
}

#[test]
fn sample_input() {
    let input = "\
        .......S.......\n\
        ...............\n\
        .......^.......\n\
        ...............\n\
        ......^.^......\n\
        ...............\n\
        .....^.^.^.....\n\
        ...............\n\
        ....^.^...^....\n\
        ...............\n\
        ...^.^...^.^...\n\
        ...............\n\
        ..^...^.....^..\n\
        ...............\n\
        .^.^.^.^.^...^.\n\
        ...............\
        ";
    let manifold = parse(input);
    assert_eq!(part1(&manifold), 21);
}

#[test]
fn merging_beams() {
    let input = "\
        ...S.\n\
        ...^.\n\
        ..^..\n\
        .^...\n\
        ...^.\n\
        .....\
        ";
    let manifold = parse(input);
    assert_eq!(part1(&manifold), 4);
}
