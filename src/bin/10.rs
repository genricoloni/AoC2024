use std::{i32, usize};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix = Vec::new();
    let mut zero_positions = Vec::new(); // Vettore per memorizzare le posizioni degli zeri

    for (row_index, line) in input.lines().enumerate() {
        let nums = line
            .trim()
            .chars()
            .enumerate() // Aggiungi l'indice di colonna
            .map(|(col_index, c)| {
                let num = c.to_digit(10).unwrap_or_else(|| panic!("Invalid number in input")) as i32;
                if num == 0 {
                    zero_positions.push((row_index, col_index)); // Salva la posizione dello zero
                }
                num
            })
            .collect::<Vec<i32>>();
        matrix.push(nums);
    }

    let index = 0;
    let mut res = 0;
    for zero in zero_positions{

        let mut visited:Vec<(usize, usize)> = Vec::new();
        res += explore(&matrix, index + 1, zero.0, zero.1 + 1, &mut visited).unwrap_or(0); // east
        if zero.1 > 0 {
            res += explore(&matrix, index + 1, zero.0, zero.1 - 1, &mut visited).unwrap_or(0); // west
        }
        if zero.0 > 0 {
            res += explore(&matrix, index + 1, zero.0 - 1, zero.1, &mut visited).unwrap_or(0); // north
        }
        res += explore(&matrix, index + 1, zero.0 + 1, zero.1, &mut visited).unwrap_or(0); // south

    }

    Some(res)
}

fn explore(map:&Vec<Vec<i32>>, value:i32, i:usize, j:usize, visited:&mut Vec<(usize,usize)>) -> Option<u32>{
    if i >= map.len(){
        return Some(0);
    }
    if j >= map[i].len(){
        return Some(0);
    }
    if map[i][j] != value{
        return Some(0);
    }
    if value == 9{
        if visited.contains(&(i,j)){
            return Some(0);
        }
        visited.push((i,j));
        return Some(1);
    }

    let east = explore(map, value+1, i, j + 1, visited).unwrap_or(0);
    let west = if j > 0 { explore(map, value+1, i, j - 1, visited).unwrap_or(0) } else { 0 };
    let north = if i > 0 { explore(map, value+1, i - 1, j, visited).unwrap_or(0) } else { 0 };
    let south = explore(map, value+1, i + 1, j, visited).unwrap_or(0);
    Some(east + west + north + south)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix = Vec::new();
    let mut zero_positions = Vec::new(); // Vettore per memorizzare le posizioni degli zeri

    for (row_index, line) in input.lines().enumerate() {
        let nums = line
            .trim()
            .chars()
            .enumerate() // Aggiungi l'indice di colonna
            .map(|(col_index, c)| {
                let num = c.to_digit(10).unwrap_or_else(|| panic!("Invalid number in input")) as i32;
                if num == 0 {
                    zero_positions.push((row_index, col_index)); // Salva la posizione dello zero
                }
                num
            })
            .collect::<Vec<i32>>();
        matrix.push(nums);
    }

    let index = 0;
    let mut res = 0;
    for zero in zero_positions{

        res += explore_part_two(&matrix, index + 1, zero.0, zero.1 + 1).unwrap_or(0); // east
        if zero.1 > 0 {
            res += explore_part_two(&matrix, index + 1, zero.0, zero.1 - 1).unwrap_or(0); // west
        }
        if zero.0 > 0 {
            res += explore_part_two(&matrix, index + 1, zero.0 - 1, zero.1).unwrap_or(0); // north
        }
        res += explore_part_two(&matrix, index + 1, zero.0 + 1, zero.1).unwrap_or(0); // south

    }

    Some(res)

}
fn explore_part_two(map:&Vec<Vec<i32>>, value:i32, i:usize, j:usize) -> Option<u32>{
    if i >= map.len(){
        return Some(0);
    }
    if j >= map[i].len(){
        return Some(0);
    }
    if map[i][j] != value{
        return Some(0);
    }
    if value == 9{

        return Some(1);
    }

    let east = explore_part_two(map, value+1, i, j + 1).unwrap_or(0);
    let west = if j > 0 { explore_part_two(map, value+1, i, j - 1).unwrap_or(0) } else { 0 };
    let north = if i > 0 { explore_part_two(map, value+1, i - 1, j).unwrap_or(0) } else { 0 };
    let south = explore_part_two(map, value+1, i + 1, j).unwrap_or(0);
    Some(east + west + north + south)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
