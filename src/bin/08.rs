advent_of_code::solution!(8);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<&str> = input.lines().collect();
    let mut antennas = Vec::new();

    // Parse the grid to find antennas
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.push((x, y, c));
            }
        }
    }

    let mut antinode_positions: HashSet<(isize, isize)> = HashSet::new();

    // Iterate through frequencies
    for freq in '0'..='z' {
        let same_freq_antennas: Vec<(usize, usize)> = antennas
            .iter()
            .filter(|&&(_, _, f)| f == freq)
            .map(|&(x, y, _)| (x, y))
            .collect();

        for i in 0..same_freq_antennas.len() {
            for j in i + 1..same_freq_antennas.len() {
                let (x1, y1) = (same_freq_antennas[i].0 as isize, same_freq_antennas[i].1 as isize);
                let (x2, y2) = (same_freq_antennas[j].0 as isize, same_freq_antennas[j].1 as isize);
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Calculate antinode positions
                let antinode1 = (x1 - dx, y1 - dy);
                let antinode2 = (x2 + dx, y2 + dy);

                // Check if antinodes are within bounds
                for antinode in [antinode1, antinode2] {
                    if antinode.0 >= 0
                        && antinode.1 >= 0
                        && (antinode.0 as usize) < grid[0].len()
                        && (antinode.1 as usize) < grid.len()
                    {
                        antinode_positions.insert(antinode);
                    }
                }
            }
        }
    }

    // Return the count of unique antinode positions
    Some(antinode_positions.len() as u32)
}
use std::collections::HashMap;

pub fn part_two(input: &str) -> Option<u32> {
    let mut field: Vec<String> = Vec::new();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    // Leggi la griglia e raccogli le posizioni delle antenne
    for (y, line) in input.lines().enumerate() {
        field.push(line.to_string());

        for (x, cell) in line.chars().enumerate() {
            if cell != '.' {
                antennas.entry(cell)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
    }

    let rows = field.len();
    let cols = field[0].len();
    let mut part1 = HashSet::new();
    let mut part2 = HashSet::new();

    // Calcola gli antinodi per ogni frequenza
    for (_, ants) in antennas.iter() {
        for (x1, y1) in ants.iter() {
            for (x2, y2) in ants.iter() {
                if *x1 == *x2 && *y1 == *y2 {
                    continue;
                }

                let dx = *x1 as isize - *x2 as isize;
                let dy = *y1 as isize - *y2 as isize;

                // Aggiungi gli antinodi per ogni antenna a distanza 2
                for mult in 1..=std::cmp::max(rows, cols) {
                    let nx = (*x2 as isize + mult as isize * dx) as usize;
                    let ny = (*y2 as isize + mult as isize * dy) as usize;

                    // Verifica se la posizione è valida nel campo
                    if nx >= cols || ny >= rows {
                        break;
                    }

                    if mult == 2 {
                        part1.insert((nx, ny));
                    }
                    part2.insert((nx, ny));
                }
            }
        }
    }

    // Ritorna il numero di antinodi unici per la parte 2
    Some(part2.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
