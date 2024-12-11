use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut array: Vec<i64> = vec![];

    for num_str in input.split_whitespace() {
        match num_str.parse::<i64>() {
            Ok(num) => array.push(num),
            Err(_) => return None,
        }
    }

    //println!("{:?}", array);

    //println!("{}", transform(array, 25));
    Some(transform(array, 25) as u64)
}

fn transform(mut array: Vec<i64>, remaining: i32) -> i64 {
    if remaining == 0 {
        return array.len().try_into().unwrap();
    }

    let mut new_array = Vec::new();

    for num in array.iter() {
        if *num == 0 {
            new_array.push(1);
            continue;
        }
        if num.to_string().len() % 2 == 0 {
            let num_str = num.to_string();
            let first_half = num_str[0..num_str.len() / 2].to_string().parse::<i64>();
            let second_half = num_str[num_str.len() / 2..].to_string().parse::<i64>();

            match first_half {
                Ok(num) => new_array.push(num),
                Err(_) => return 0,
            }
            match second_half {
                Ok(num) => new_array.push(num),
                Err(_) => return 0,
            }
        } else {
            new_array.push(num * 2024);
        }
    }

    array = new_array;
    //println!("{:?}", array);


    transform(array, remaining - 1)
}


fn part_two(input: &str) -> Option<u128> {
    let mut stones: HashMap<u64, u128> = input
        .trim()
        .split_ascii_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect();
    for _ in 0..75 {
        for (stone, n) in stones.drain().collect::<Vec<_>>() {
            let mut insert = |s| {
                stones.entry(s).and_modify(|x| *x += n).or_insert(n);
            };
            if stone == 0 {
                insert(1);
            } else {
                match (stone as f32).log10().floor() as u32 + 1 {
                    digits if digits % 2 == 0 => {
                        insert(stone / 10u64.pow(digits / 2));
                        insert(stone % 10u64.pow(digits / 2));
                    }
                    _ => insert(stone * 2024),
                }
            }
        }
    }
    Some(stones.values().copied().sum::<u128>())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
