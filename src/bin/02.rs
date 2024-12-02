advent_of_code::solution!(2);


fn check_order(input: &Vec<i32>) -> bool {
    let mut input_sorted = input.clone();
    input_sorted.sort();

    let rev: Vec<i32> = input_sorted.clone().into_iter().rev().collect();

    *input == input_sorted || *input == rev
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = 0;

    for line in input.lines(){
        let nums = line.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if !check_order(&nums){
            continue;
        }

        let mut tmp = 1;
        //iter using index from 0
        for i in 0..nums.len()-1{
            if (nums[i] - nums [i+1]).abs() < 1 || (nums[i] - nums [i+1]).abs() > 3{
                tmp = 0;
            }
        }
        res += tmp;

    }
    Some(res as u32)
}

//true if vec is mainly decreasing
//false if vec is mainly increasing
fn check_orientation(input: &Vec<i32>) -> bool {
    let mut orientation = 0;

    for i in 0..input.len()-1 {
        orientation += (input[i] - input[i+1]).signum();
    }

    orientation > 0
}

fn check_no_errors(nums: &Vec<i32>) -> bool {
    if !check_order(nums){
        return false;
    }
    for i in 0..nums.len()-1{
        if (nums[i] - nums [i+1]).abs() < 1 || (nums[i] - nums [i+1]).abs() > 3{
            return false;
        }
    }
    true


}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = 0;

    for line in input.lines(){
        let mut nums = line.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();



    if check_no_errors(&nums){
        res += 1;
        continue;
    } else {
                //print!("Seq: {:?}", nums);
        if check_orientation(&nums) == false{
            nums.reverse();
        }
        let mut i = 0;

        while i + 2 <= nums.len(){
            if nums[i] <= nums[i+1] || (nums[i] - nums [i+1]) > 3{
                let mut nums_clone = nums.clone();
                nums_clone.remove(i);
                let mut nums_clone2 = nums.clone();
                nums_clone2.remove(i+1);
                res += (check_no_errors(&nums_clone) || check_no_errors(&nums_clone2)) as i32;

                break;
            }
            i+=1
        }
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
        assert_eq!(result, Some(2));
        }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
