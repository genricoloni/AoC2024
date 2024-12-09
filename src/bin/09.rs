advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut array: Vec<i32> = vec![];
    let mut index = 0;
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            if let Some(digit) = c.to_digit(10) {
                for _j in 0..digit {
                    array.push(index);
                }
                index += 1;
            }
        } else {
            if let Some(digit) = c.to_digit(10) {
                for _j in 0..digit {
                    array.push(-1);
                }
            }
        }
    }
    //println!("{:?}", array);

    let mut i = 0;
    let mut j = array.len() - 1;
    let mut res: i64 = 0;

    while i <= j {
        if array[i] != -1 {
            res += i as i64 * array[i] as i64;
            //println!("{}*{}={}", i, array[i], i as i32*array[i]);

            i += 1;
            continue;
        }
        if array[j] == -1 {
            j -= 1;
            continue;
        }
        array[i] = array[j];
        array[j] = -1;
        res += i as i64 * array[i] as i64;
        //println!("{}*{}={}", i, array[i], i as i32*array[i]);
        i += 1;
        j -= 1;
    }
    //println!("i:{}, j:{}", i,j);
    /*for (i,c) in array.iter().enumerate(){
        if *c == -1 {
            print!(".");
        } else {
            print!("{}", c);
        }
    }*/
    Some(res.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut array: Vec<i32> = vec![];
    let mut index = 0;
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            if let Some(digit) = c.to_digit(10) {
                for _j in 0..digit {
                    array.push(index);
                }
                index += 1;
            }
        } else {
            if let Some(digit) = c.to_digit(10) {
                for _j in 0..digit {
                    array.push(-1);
                }
            }
        }
    }
    let mut j = array.len() - 1;
    let mut res: i64 = 0;

    let mut visited:Vec<i32> = vec![];

    while j > 0 {
        if array[j] == -1 || visited.contains(&array[j]){
            j -= 1;
            continue;
        }

        let mut buf = 0;
        while array[j-buf] == array[j] && j-buf > 0{
            buf += 1;
        }
        if j < buf {
            break;
        }
        //println!("{:?}", &array[j-buf+1..j+1]);
        //println!("SPACE NEEDED: {}", buf);

        let indexes = find_sequence(&array, -1, buf);

        if indexes == None{
            //no enoguh space
            visited.push(array[j]);
            //println!("NO SPACE AVAILABLE");


            j -= buf;
            continue;
        }
        //swap element
        let (mut start, end) = indexes.unwrap();
        if  start >= j {
            visited.push(array[j]);
            //println!("NO SPACE AVAILABLE");

            j -= buf;
            continue;
        }
        visited.push(array[j]);

        while start <= end {
            array[start] = array[j];
            array[j] = -1;
            j -= 1;
            start += 1;
        }
        //println!("{:?}", array);

    }

    for (i,c) in array.iter().enumerate(){
        if *c != -1 {
            res += i as i64 * array[i] as i64;
        }
    }


    Some(res.try_into().unwrap())
}

fn find_sequence(vec: &[i32], target: i32, target_length: usize) -> Option<(usize, usize)> {
    let mut count = 0;
    let mut start_index = None;

    for (i, &val) in vec.iter().enumerate() {
        if val == target {
            if start_index.is_none() {
                start_index = Some(i);
            }
            count += 1;

            if count == target_length {
                return Some((start_index.unwrap(), i));
            }
        } else {
            count = 0;
            start_index = None;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
