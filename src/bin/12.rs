use std::collections::{HashMap, HashSet, VecDeque};


advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let rows = lines.len();
    let cols = lines[0].len();

    // Convert each line into a vec of characters for easy indexing
    let grid: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect()).collect();

    // Track which cells we've already visited
    let mut visited = vec![vec![false; cols]; rows];

    let mut total_cost = 0u32;

    // For checking 4-directional neighbors
    let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let letter = grid[r][c];
                let mut queue = VecDeque::new();
                queue.push_back((r, c));
                visited[r][c] = true;

                let mut area = 0u32;
                let mut perimeter = 0u32;

                while let Some((rr, cc)) = queue.pop_front() {
                    area += 1;

                    // Check each of the 4 neighbors
                    for (dr, dc) in directions.iter() {
                        let nr = rr as i32 + dr;
                        let nc = cc as i32 + dc;
                        // If neighbor is out of bounds, perimeter += 1
                        if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                            perimeter += 1;
                        } else {
                            // Otherwise, check if it's the same letter
                            let (nr_usize, nc_usize) = (nr as usize, nc as usize);
                            if grid[nr_usize][nc_usize] != letter {
                                perimeter += 1;
                            } else if !visited[nr_usize][nc_usize] {
                                visited[nr_usize][nc_usize] = true;
                                queue.push_back((nr_usize, nc_usize));
                            }
                        }
                    }
                }

                total_cost += area * perimeter;
            }
        }
    }

    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return None;
    }

    // Costruiamo la griglia di caratteri
    let rows = lines.len();
    let cols = lines[0].len();
    let grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    // Tabella visited per BFS
    let mut visited = vec![vec![false; cols]; rows];

    let mut total_price = 0u32;

    // Direzioni 4-conn per BFS
    let directions = [(0,1), (0,-1), (1,0), (-1,0)];

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                // Lettera di questa regione
                let letter = grid[r][c];
                // Partiamo la BFS
                let mut queue = VecDeque::new();
                queue.push_back((r as i32, c as i32));
                visited[r][c] = true;

                let mut area = 0u32;
                // In questo HashSet salveremo gli spigoli di confine in "corner coords"
                let mut boundary_edges = HashSet::new();

                while let Some((rr, cc)) = queue.pop_front() {
                    area += 1;

                    // Esaminiamo i 4 vicini
                    for (dr, dc) in directions {
                        let nr = rr + dr;
                        let nc = cc + dc;
                        // Controllo bounds
                        if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                            // out-of-bounds => spigolo di confine
                            add_edge(&mut boundary_edges, rr, cc, dr, dc);
                        } else {
                            // in bounds
                            let (ur, uc) = (nr as usize, nc as usize);
                            if grid[ur][uc] != letter {
                                // lettera diversa => spigolo di confine
                                add_edge(&mut boundary_edges, rr, cc, dr, dc);
                            } else if !visited[ur][uc] {
                                visited[ur][uc] = true;
                                queue.push_back((nr, nc));
                            }
                        }
                    }
                }

                // Ora calcoliamo quanti "lati" complessivi ha questa regione,
                // considerando i segmenti orizzontali/verticali nel boundary_edges.
                let sides = count_sides(&boundary_edges);
                let price = area * sides;
                total_price += price;
            }
        }
    }

    Some(total_price)
}

/// Aggiunge lo spigolo di confine corrispondente al lato di cella (r,c)
/// in direzione (dr,dc).
///
/// Se (dr,dc) = (-1,0), vuol dire il "top" di cella (r,c):
///   corners = (r,c) .. (r,c+1)
/// Se (dr,dc) = (1,0), vuol dire il "bottom" di cella (r,c):
///   corners = (r+1,c) .. (r+1,c+1)
/// Se (dr,dc) = (0,-1), vuol dire "left":
///   corners = (r,c) .. (r+1,c)
/// Se (dr,dc) = (0,1), vuol dire "right":
///   corners = (r,c+1) .. (r+1,c+1)
fn add_edge(boundary_edges: &mut HashSet<((i32,i32),(i32,i32))>,
            r: i32, c: i32,
            dr: i32, dc: i32)
{
    let (start, end) = match (dr, dc) {
        (-1, 0) => ((r, c), (r, c+1)),         // top
        (1, 0)  => ((r+1, c), (r+1, c+1)),     // bottom
        (0, -1) => ((r, c), (r+1, c)),         // left
        (0, 1)  => ((r, c+1), (r+1, c+1)),     // right
        _ => panic!("Unexpected direction: ({}, {})", dr, dc),
    };

    // Normalizziamo l'edge affinché start < end in ordine lessicografico
    let edge = if start < end { (start, end) } else { (end, start) };
    boundary_edges.insert(edge);
}

/// Conta quanti "lati" totali formano il recinto.
/// Usa la tecnica "merge-and-count" per segmenti orizzontali e verticali separatamente,
/// evitando di creare un singolo lato se c'è un buco in mezzo.
fn count_sides(boundary_edges: &HashSet<((i32,i32),(i32,i32))>) -> u32 {
    // Raggruppiamo i segmenti orizzontali per riga,
    // i segmenti verticali per colonna.
    let mut horizontals: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut verticals:   HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for &(start, end) in boundary_edges {
        let ((r1, c1), (r2, c2)) = (start, end);
        if r1 == r2 {
            // orizzontale
            let row = r1;
            let col_start = c1.min(c2);
            let col_end   = c1.max(c2);
            horizontals.entry(row).or_default().push((col_start, col_end));
        } else if c1 == c2 {
            // verticale
            let col = c1;
            let row_start = r1.min(r2);
            let row_end   = r1.max(r2);
            verticals.entry(col).or_default().push((row_start, row_end));
        } else {
            // Non dovrebbero esserci diagonali
            panic!("Found a non-rectilinear edge: ({:?},{:?})", start, end);
        }
    }

    // Funzione d'aiuto: unisce segmenti 1D contigui o sovrapposti.
    fn merge_and_count(mut segments: Vec<(i32,i32)>) -> u32 {
        if segments.is_empty() {
            return 0;
        }
        // ordiniamo per start
        segments.sort_by_key(|&(s, _)| s);

        let mut count = 0;
        let mut cur_start = segments[0].0;
        let mut cur_end   = segments[0].1;

        for &(s, e) in segments.iter().skip(1) {
            if s <= cur_end {
                // i segmenti si toccano o si sovrappongono
                // estendiamo il blocco
                if e > cur_end {
                    cur_end = e;
                }
            } else {
                // c'è un buco => chiudiamo un blocco
                count += 1;
                // apriamone uno nuovo
                cur_start = s;
                cur_end   = e;
            }
        }
        // chiudiamo l'ultimo blocco
        count += 1;
        count
    }

    // Sommiamo i "blocchi" orizzontali e verticali
    let mut total_sides = 0;
    // orizzontali
    for (_, segs) in horizontals {
        total_sides += merge_and_count(segs);
    }
    // verticali
    for (_, segs) in verticals {
        total_sides += merge_and_count(segs);
    }

    total_sides
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(368));
    }
}
