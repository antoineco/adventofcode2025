pub fn parse(input: &str) -> Vec<Option<()>> {
    input
        .chars()
        .filter_map(|c| match c {
            '@' => Some(Some(())),
            '.' => Some(None),
            _ => None,
        })
        .collect()
}

const MAX_ADJACENT_ROLLS: u32 = 4;

pub fn part1(rolls: &[Option<()>]) -> usize {
    let size = rolls.len().isqrt();

    let shift_below = size as i32;
    let shift_above = -shift_below;
    let shift_right = 1;
    let shift_left = -shift_right;

    rolls
        .iter()
        .enumerate()
        .filter(|(roll, x)| {
            if x.is_none() {
                return false;
            }

            let mut n_adj_rolls = 0;
            'outer: for sft_h in [shift_left, 0, shift_right] {
                let h_pos = (roll % size) as i32 + sft_h;
                if h_pos < 0 || h_pos as usize >= size {
                    continue;
                }
                for sft_v in [shift_above, 0, shift_below] {
                    if (sft_v, sft_h) == (0, 0) {
                        continue;
                    }
                    let pos = *roll as i32 + sft_v + sft_h;
                    if pos < 0 || pos as usize >= size * size {
                        continue;
                    }
                    if rolls[pos as usize].is_some() {
                        n_adj_rolls += 1;
                        if n_adj_rolls == MAX_ADJACENT_ROLLS {
                            break 'outer;
                        }
                    }
                }
            }

            n_adj_rolls < MAX_ADJACENT_ROLLS
        })
        .count()
}

pub fn part2(_: &[Option<()>]) -> usize {
    0
}

#[test]
fn sample_input() {
    let input = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.\
        ";
    let rolls = parse(input);
    assert_eq!(part1(&rolls), 13);
}
