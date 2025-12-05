use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut iter_lines = input.lines();

    let mut fresh_ranges: Vec<(u64, u64)> = vec![];
    for l in iter_lines.by_ref() {
        if l.is_empty() {
            break;
        }
        let mut iter_ids = l.iter_unsigned().take(2);
        fresh_ranges.push((iter_ids.next().unwrap(), iter_ids.next().unwrap()));
    }

    let available = iter_lines.map(|l| l.unsigned()).collect();

    (fresh_ranges, available)
}

pub fn part1(db: &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    let (fresh_ranges, available) = db;
    let fresh_ranges = merged_ranges(fresh_ranges);

    available
        .iter()
        .filter(|&id| {
            for (begin, end) in &fresh_ranges {
                if id >= begin && id <= end {
                    return true;
                }
            }
            false
        })
        .count() as u64
}

pub fn part2(_: &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    0
}

fn merged_ranges(rngs: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut merged_rngs = Vec::with_capacity(rngs.len());

    let mut rngs = rngs.to_owned();
    rngs.sort();

    let mut nxt_end = rngs[0].1;
    let (mut i, mut j) = (0, 1);
    loop {
        let (i_begin, _) = rngs[i];
        if j == rngs.len() {
            merged_rngs.push((i_begin, nxt_end));
            break merged_rngs;
        }

        let (j_begin, j_end) = rngs[j];
        if j_begin > nxt_end {
            merged_rngs.push((i_begin, nxt_end));
            i = j;
        }
        if j_end > nxt_end {
            nxt_end = j_end;
        }
        j += 1;
    }
}

#[test]
fn sample_input() {
    let input = "\
        3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17\n\
        32\
        ";
    let db = parse(input);
    assert_eq!(part1(&db), 3);
}

#[test]
fn merge_ranges() {
    assert_eq!(
        merged_ranges(&[(3, 5), (10, 14), (16, 20), (12, 18)]),
        vec![(3, 5), (10, 20)]
    );

    assert_eq!(
        merged_ranges(&[(3, 5), (10, 12), (16, 20), (12, 18)]),
        //                           ^
        vec![(3, 5), (10, 20)]
    );

    assert_eq!(
        merged_ranges(&[(3, 5), (10, 11), (16, 20), (12, 18)]),
        //                           ^
        vec![(3, 5), (10, 11), (12, 20)]
    );

    assert_eq!(
        merged_ranges(&[(3, 5), (10, 10), (16, 20), (12, 18)]),
        //                           ^
        vec![(3, 5), (10, 10), (12, 20)]
    );

    assert_eq!(
        merged_ranges(&[(3, 5), (10, 14), (19, 20), (12, 18)]),
        //                                 ^
        vec![(3, 5), (10, 18), (19, 20)]
    );

    assert_eq!(
        merged_ranges(&[(3, 11), (10, 14), (16, 20), (12, 18)]),
        //                  ^
        vec![(3, 20)]
    );

    assert_eq!(
        merged_ranges(&[(3, 5), (10, 14), (16, 20), (17, 18)]),
        //                                           ^
        vec![(3, 5), (10, 14), (16, 20)]
    );

    assert_eq!(
        merged_ranges(&[(3, 5), (10, 14), (16, 20), (12, 13)]),
        //                                               ^
        vec![(3, 5), (10, 14), (16, 20)]
    );
}
