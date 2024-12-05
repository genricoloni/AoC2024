advent_of_code::solution!(5);
use std::collections::HashMap;

// Part One solution
pub fn part_one(input: &str) -> Option<u32> {
    // Split the input into rules and pages parts
    let rules_input = input.split("\n\n").collect::<Vec<&str>>()[0];
    let pages_input = input.split("\n\n").collect::<Vec<&str>>()[1];

    // Create a hashmap to store page dependencies
    let mut page_dependencies: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut result = 0;

    // Parse the rules and populate the dependencies map
    for rule in rules_input.lines() {
        let parts: Vec<&str> = rule.split('|').collect();
        // Add the dependency relationship to the map
        page_dependencies.entry(parts[1].parse::<i32>().unwrap())
            .or_insert(Vec::new())
            .push(parts[0].parse::<i32>().unwrap());
    }

    // Process each page
    for page in pages_input.lines() {
        let mut forbidden: Vec<i32> = Vec::new();
        // Convert the page into a vector of page numbers
        let page_numbers: Vec<i32> = page.split(",").map(|s| s.parse().unwrap()).collect();

        let mut is_valid = true;

        // Check if any number violates the order by appearing after a forbidden number
        for &page_number in &page_numbers {
            if forbidden.contains(&page_number) {
                is_valid = false;
                break;
            }
            // Add all dependencies for the current page number to the forbidden list
            if let Some(dependencies) = page_dependencies.get(&page_number) {
                forbidden.extend(dependencies);
            }
        }

        // If the page is valid, add the middle number of the page to the result
        if is_valid {
            result += page_numbers[page_numbers.len() / 2];
        }
    }

    // Return the result as a wrapped Option
    Some(result.try_into().unwrap())
}

// Part Two solution
pub fn part_two(input: &str) -> Option<u32> {
    // Split the input into rules and pages parts
    let rules_input = input.split("\n\n").collect::<Vec<&str>>()[0];
    let pages_input = input.split("\n\n").collect::<Vec<&str>>()[1];

    // Create a hashmap to store page dependencies
    let mut page_dependencies: HashMap<i32, Vec<i32>> = HashMap::new();

    // Parse the rules and populate the dependencies map
    for rule in rules_input.lines() {
        let parts: Vec<&str> = rule.split('|').collect();
        page_dependencies.entry(parts[1].parse::<i32>().unwrap())
            .or_insert(Vec::new())
            .push(parts[0].parse::<i32>().unwrap());
    }

    let mut result = 0;

    // Process each page
    for page in pages_input.lines() {
        // Convert the page into a vector of page numbers
        let mut page_numbers: Vec<i32> = page.split(",").map(|s| s.parse().unwrap()).collect();

        // Sort the page numbers based on their dependencies
        page_numbers.sort_by(|a, b| {
            // If b is dependent on a, it should come first
            if page_dependencies.get(b).unwrap_or(&vec![]).contains(&a) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        // If the page is not already sorted, add the middle number of the sorted page to the result
        if page_numbers != page
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>()
        {
            let middle_value = page_numbers[page_numbers.len() / 2];
            result += middle_value;
        }
    }

    // Return the result as a wrapped Option
    Some(result.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for Part One
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // Check if the result matches the expected value
        assert_eq!(result, Some(143));
    }

    // Test for Part Two
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // Check if the result matches the expected value
        assert_eq!(result, Some(123));
    }
}
