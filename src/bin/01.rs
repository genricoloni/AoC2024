advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    //parse the inputs

    //define two lists whose length is the length of the input
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    //iterate over the input
    for line in input.lines() {
        //split the line by spaces
        let parts = line.split("   ");
        let collect: Vec<&str> = parts.collect();

        list1.push(collect[0].parse::<i32>().unwrap());
        list2.push(collect[1].parse::<i32>().unwrap());
    }

    //sort the lists

    list1.sort();
    list2.sort();

    let mut res = 0;

    for i in 0..list1.len() {
        res += (list1[i] - list2[i]).abs();
    }

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map1: HashMap<i32, i32> = HashMap::new();
    let mut map2: HashMap<i32, i32> = HashMap::new();

    //iterate over the input
    for line in input.lines() {
        //split the line by spaces
        let parts = line.split("   ");
        let collect: Vec<&str> = parts.collect();

        *map1.entry(collect[0].parse::<i32>().unwrap()).or_insert(0) += 1;
        *map2.entry(collect[1].parse::<i32>().unwrap()).or_insert(0) += 1;
    }

    let mut res = 0;

    for key in &map1 {
        res += map2.get(key.0).unwrap_or(&0) * key.0;
    }

    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }
}
