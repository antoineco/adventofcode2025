use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|range| {
            let mut iter = range.iter_unsigned();
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}

pub fn part1(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .map(|(first, last)| {
            (*first..=*last)
                .map(|id| {
                    let id_str = id.to_string();
                    let len = id_str.len();
                    if id_str
                        .chars()
                        .take(len / 2)
                        .eq(id_str.chars().skip(len / 2))
                    {
                        id
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

pub fn part2(_: &[(u64, u64)]) -> u64 {
    0
}

#[test]
fn sample_input() {
    let input = "\
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124\
        ";
    let ranges = parse(input);
    assert_eq!(part1(&ranges), 1227775554);
}
