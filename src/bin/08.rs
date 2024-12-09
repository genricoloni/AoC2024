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


pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
