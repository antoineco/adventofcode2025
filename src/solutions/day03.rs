use std::cmp::max;

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

pub fn part1(banks: &[Vec<u8>]) -> u64 {
    banks
        .iter()
        .map(|b| {
            let mut j_max = 0;
            let mut j_max_next = 0;
            b.iter().zip(b.iter().skip(1)).for_each(|(j, j_next)| {
                j_max_next = if j > &j_max {
                    j_max = *j;
                    *j_next
                } else {
                    max(*j_next, j_max_next)
                };
            });
            (j_max * 10 + j_max_next) as u64
        })
        .sum()
}

pub fn part2(_: &[Vec<u8>]) -> u64 {
    0
}

#[test]
fn sample_input() {
    let input = "\
        987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111\
        ";
    let jolts = parse(input);
    assert_eq!(part1(&jolts), 98 + 89 + 78 + 92);
}

#[test]
fn max_at_boundaries() {
    let input = "\
        900000000000000\n\
        000000000000009\
        ";
    let banks = parse(input);
    assert_eq!(part1(&banks), 90 + 9);
}
