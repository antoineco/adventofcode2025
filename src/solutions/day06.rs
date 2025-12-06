use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let mut iter_lines = input.lines();

    let operations: Vec<_> = iter_lines
        .next_back()
        .unwrap()
        .matches(['+', '*'])
        .collect();

    let mut problems: Vec<_> = (0..operations.len()).map(|_| vec![]).collect();
    for l in iter_lines {
        l.iter_unsigned().enumerate().for_each(|(i, x)| {
            problems[i].push(x);
        });
    }

    (problems, operations)
}

pub fn part1(worksheet: &(Vec<Vec<u64>>, Vec<&str>)) -> u64 {
    let (problems, operations) = worksheet;

    problems
        .iter()
        .enumerate()
        .map(|(i, p)| match operations[i] {
            "*" => p.iter().product::<u64>(),
            _ => p.iter().sum(),
        })
        .sum()
}

pub fn part2(_: &(Vec<Vec<u64>>, Vec<&str>)) -> u64 {
    0
}

#[test]
fn sample_input() {
    let input = "".to_owned()
        + "123 328  51 64\n"
        + " 45 64  387 23\n"
        + "  6 98  215 314\n"
        + "*   +   *   +  ";
    let worksheet = parse(&input);
    assert_eq!(part1(&worksheet), 33210 + 490 + 4243455 + 401);
}
