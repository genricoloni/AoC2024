advent_of_code::solution!(7);

#[derive(Debug, Clone)]
struct CalibrationState {
    target: i64,
    numbers: Vec<i64>,
}

pub fn part_one(input: &str) -> Option<i64> {
    let parsed: Vec<CalibrationState> = input
        .lines()
        .map(|line| {
            let split = line.chars().position(|c| c == ':').unwrap();
            let target = line[..split].parse().unwrap();
            let numbers = line[split + 1..]
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>();
            CalibrationState { target, numbers }
        })
        .collect();

    let mut total = 0;

    for state in parsed {
        if check_possible(state.target, &state.numbers) {
            total += state.target;
        }
    }

    Some(total)
}

fn check_possible(target: i64, numbers: &[i64]) -> bool {
    let mut stack = vec![(numbers[0], 1)];

    while let Some((acc, ptr)) = stack.pop() {
        if ptr == numbers.len() {
            if acc == target {
                return true;
            }
            continue;
        }

        let num = numbers[ptr];

        if acc + num <= target {
            stack.push((acc + num, ptr + 1));
        }

        if acc * num <= target {
            stack.push((acc * num, ptr + 1));
        }
    }

    false
}


pub fn part_two(input: &str) -> Option<i64> {
    let data: Vec<(i64, Vec<i64>)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let target = parts.next().unwrap().trim().parse().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            (target, numbers)
        })
        .collect();

    let mut total = 0;

    for (target, numbers) in data {
        let result = find_valid_combinations(target, &numbers);
        if result != 0 {
            total += result;
        }
    }

    Some(total)
}

fn find_valid_combinations(target: i64, numbers: &[i64]) -> i64 {
    let mut unprocessed = vec![(0, 0)];

    while let Some((accumulator, pointer)) = unprocessed.pop() {
        if pointer == numbers.len() {
            if accumulator == target {
                return target;
            }
            continue;
        }

        if accumulator > target {
            continue;
        }
        unprocessed.push((accumulator + numbers[pointer], pointer + 1));
        unprocessed.push((accumulator * numbers[pointer], pointer + 1));
        unprocessed.push((concatenate(accumulator, numbers[pointer]), pointer + 1));
    }

    0
}

fn concatenate(accumulator: i64, next: i64) -> i64 {
    let accumulator_str = accumulator.to_string();
    let next_str = next.to_string();
    let concatenated = accumulator_str + &next_str;
    concatenated.parse().unwrap();
    kk
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
