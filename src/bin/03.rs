advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    // Regex per catturare i numeri dentro la parentesi
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut res = 0;

    // Troviamo tutte le occorrenze
    for cap in re.find_iter(input) {
        // Otteniamo le catture
        let captures = re.captures(cap.as_str()).unwrap();

        // Estraiamo i numeri dai gruppi di cattura
        let num1: i32 = captures[1].parse().unwrap();
        let num2: i32 = captures[2].parse().unwrap();

        println!("Trovati {} * {} = {}", num1, num2, num1 * num2);
        res += num1 * num2;
    }

    println!("Alla fine: {}", res);

    Some(res as u32)
}


pub fn part_two(input: &str) -> Option<u32> {
    // Le regex per i vari pattern
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
        let mut res = 0;

        let mut enable = 1;

        for cap in re.find_iter(input) {
            // Otteniamo le catture
            println!("Tovato:{:?}", cap.as_str());

            match &cap.as_str()[..4] {
                "mul(" => {
                    if enable == 0 {
                        continue;
                    }
                    let captures = re.captures(cap.as_str()).unwrap();
                    let num1: i32 = captures[1].parse().unwrap();
                    let num2: i32 = captures[2].parse().unwrap();
                    res += num1 * num2;
                },
                "do()" => {
                    enable = 1;
                },
                "don'" => {
                    enable = 0;
                },
                _ => {}
            }
        }


    Some(res as u32)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48 as u32));
    }
}
