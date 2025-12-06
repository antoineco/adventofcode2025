use crate::util::parse::ParseOps;

pub fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<Vec<String>>, Vec<&str>) {
    let mut iter_lines = input.lines();

    let operations: Vec<_> = iter_lines
        .next_back()
        .unwrap()
        .matches(['+', '*'])
        .collect();

    let mut problems1: Vec<_> = (0..operations.len()).map(|_| vec![]).collect();
    for l in iter_lines {
        l.iter_unsigned().enumerate().for_each(|(i, x)| {
            problems1[i].push(x);
        });
    }

    let max_line_len = input.lines().next_back().unwrap().len();
    let problems_length: Vec<_> = input
        .lines()
        .next_back()
        .unwrap()
        .match_indices(['+', '*'])
        .skip(1) // first op starts at index 0
        .chain(vec![(max_line_len + 1, "")]) // end with a fake op to include last problem
        .scan(0, |prev_op_idx, (op_idx, _)| {
            let len = op_idx - *prev_op_idx - 1; // do not count space separator
            *prev_op_idx = op_idx;
            Some(len)
        })
        .collect();

    let mut problems2: Vec<_> = problems_length
        .iter()
        // Pre-populate empty digit strings for each problem:
        //   [ "", "", "" ], [ "", "", "" ], ...
        .map(|&n| vec!["".to_owned(); n])
        .collect();

    input.lines().for_each(|l| {
        let mut iter_chars = l.chars();
        problems_length.iter().enumerate().for_each(|(p, &n)| {
            // Grow pre-populated digit strings problem by problem and character by character:
            //   [ "1", "2", "3" ], [ "", "", "" ], ...
            //   [ "1", "2", "3" ], [ "3", "2", "8" ], ...
            (0..n).for_each(|i| {
                if let Some(c) = iter_chars.next()
                    && c.is_ascii_digit()
                {
                    problems2[p][i].push(c);
                }
            });
            iter_chars.next(); // consume space separator, no-op at EOL
        });
        // Each newly parsed line grows digit strings further:
        //   [ "1", "2",  "3"   ], [ "3",   "2",   "8" ], ...
        //   [ "1", "24", "35"  ], [ "36",  "24",  "8" ], ...
        //   [ "1", "24", "356" ], [ "369", "248", "8" ], ...
    });

    (problems1, problems2, operations)
}

pub fn part1(worksheet: &(Vec<Vec<u64>>, Vec<Vec<String>>, Vec<&str>)) -> u64 {
    let (problems, _, operations) = worksheet;

    problems
        .iter()
        .enumerate()
        .map(|(i, p)| match operations[i] {
            "*" => p.iter().product::<u64>(),
            _ => p.iter().sum(),
        })
        .sum()
}

pub fn part2(worksheet: &(Vec<Vec<u64>>, Vec<Vec<String>>, Vec<&str>)) -> u64 {
    let (_, problems, operations) = worksheet;

    problems
        .iter()
        .enumerate()
        .map(|(i, p)| match operations[i] {
            "*" => p.iter().map(|x| x.parse::<u64>().unwrap()).product::<u64>(),
            _ => p.iter().map(|x| x.parse::<u64>().unwrap()).sum(),
        })
        .sum()
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
    assert_eq!(part2(&worksheet), 1058 + 3253600 + 625 + 8544);
}
