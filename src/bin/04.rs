advent_of_code::solution!(4);

const TARGET: &[char] = &['X', 'M', 'A', 'S'];

pub fn part_one(input: &str) -> Option<u32> {
    // Convert input to a Vec<Vec<char>>
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut res = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'X' {
                //println!("Found: X at {},{}. Res is {}", i, j, res);

                // Call `check` with all 8 directions
                res += check(1, i, j, Direction::Up, &matrix);
                res += check(1, i, j, Direction::Down, &matrix);
                res += check(1, i, j, Direction::Left, &matrix);
                res += check(1, i, j, Direction::Right, &matrix);
                res += check(1, i, j, Direction::UpRight, &matrix);
                res += check(1, i, j, Direction::UpLeft, &matrix);
                res += check(1, i, j, Direction::DownRight, &matrix);
                res += check(1, i, j, Direction::DownLeft, &matrix);
            }
        }
    }

    Some(res.try_into().unwrap()) // Return the result
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

fn check(mut a: usize, i: usize, j: usize, dir: Direction, matrix: &Vec<Vec<char>>) -> i32 {
    // Convert indices to i32 for safe arithmetic
    let (mut i, mut j) = (i as i32, j as i32);

    // Adjust indices based on direction
    match dir {
        Direction::Up => i -= 1,
        Direction::Down => i += 1,
        Direction::Left => j -= 1,
        Direction::Right => j += 1,
        Direction::UpRight => {
            i -= 1;
            j += 1;
        }
        Direction::UpLeft => {
            i -= 1;
            j -= 1;
        }
        Direction::DownRight => {
            i += 1;
            j += 1;
        }
        Direction::DownLeft => {
            i += 1;
            j -= 1;
        }
    }

    // Convert indices back to usize and check bounds
    if i < 0 || j < 0 || i as usize >= matrix.len() || j as usize >= matrix[i as usize].len() {
        return 0;
    }

    let (i, j) = (i as usize, j as usize);

    // Check if the current character matches
    if matrix[i][j] == TARGET[a] {
        //println!("Checked: {}, {} with direction {:?}", matrix[i][j], TARGET[a], dir);

        // If the entire target is found, return 1
        if TARGET[a] == 'S' {
            return 1;
        } else {
            a += 1;

            // Continue checking recursively in the same direction
            return check(a, i, j, dir, matrix);
        }
    }

    0
}

fn check_second_part(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> i32 {
    // Caratteri da verificare
    let mut chars1: Vec<char> = vec!['M', 'S'];
    let mut chars2: Vec<char> = vec!['M', 'S'];

    // Funzione per rimuovere un carattere dal Vec, se esiste
    fn remove_single_if_present(chars: &mut Vec<char>, target: char) {
        if let Some(pos) = chars.iter().position(|&c| c == target) {
            chars.remove(pos);
        }
    }

    //check the UL-DR diagonal
    if i > 0 && j > 0 {
        remove_single_if_present(&mut chars1, matrix[i - 1][j - 1]);
    }
    if i + 1 < matrix.len() && j + 1 < matrix[i].len() {
        remove_single_if_present(&mut chars1, matrix[i + 1][j + 1]);
    }

    //check the UP-DL
    if i > 0 && j + 1 < matrix[i].len() {
        remove_single_if_present(&mut chars2, matrix[i - 1][j + 1]);
    }
    if i + 1 < matrix.len() && j > 0 {
        remove_single_if_present(&mut chars2, matrix[i + 1][j - 1]);
    }

    // Se il Vec è vuoto, tutti i caratteri sono stati trovati e rimossi
    if chars1.is_empty() && chars2.is_empty() {
        1
    } else {
        0
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // Convert input to a Vec<Vec<char>>
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut res = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'A' {
                res += check_second_part(i, j, &matrix);
            }
        }
    }

    Some(res.try_into().unwrap()) // Return the result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
