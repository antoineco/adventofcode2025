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

pub fn part2(banks: &[Vec<u8>]) -> u64 {
    const JOLT_DIGITS: usize = 12;

    banks
        .iter()
        .map(|b| {
            (1..=JOLT_DIGITS)
                .scan((0, 0), |(j_max, j_max_pos), step| {
                    *j_max = 0;
                    let skip = if step == 1 { 0 } else { *j_max_pos + 1 };
                    b.iter()
                        .enumerate()
                        .skip(skip)
                        .take(b.len() - skip - (JOLT_DIGITS - step))
                        .for_each(|(i, j)| {
                            if j > j_max {
                                *j_max = *j;
                                *j_max_pos = i;
                            }
                        });
                    Some((*j_max as u64) * 10u64.pow((JOLT_DIGITS - step) as u32))
                })
                .sum::<u64>()
        })
        .sum()
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
    assert_eq!(
        part2(&jolts),
        987654321111 + 811111111119 + 434234234278 + 888911112111
    );
}

#[test]
fn max_at_boundaries() {
    let input = "\
        911111111111111\n\
        111111111111119\
        ";
    let banks = parse(input);
    assert_eq!(part1(&banks), 91 + 19);
    assert_eq!(part2(&banks), 911111111111 + 111111111119);
}
