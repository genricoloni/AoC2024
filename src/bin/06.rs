use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut indexes = (0, 0);
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for i in 0..matrix.len() {
        if let Some(pos) = matrix[i].iter().position(|&r| r == '^') {
            indexes.0 = i;
            indexes.1 = pos;
            break;
        }
    }

    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir_index = 0;
    let mut res = 0;

    while true {
        while matrix[indexes.0][indexes.1] != '#' {
            matrix[indexes.0][indexes.1] = 'X';
            indexes.0 = (indexes.0 as isize + dirs[dir_index].0) as usize;
            indexes.1 = (indexes.1 as isize + dirs[dir_index].1) as usize;

            // check if it's out of bound
            if indexes.0 >= matrix.len() || indexes.1 >= matrix[0].len() {
                for i in 0..matrix.len() {
                    res += matrix[i].iter().filter(|&&c| c == 'X').count() as u32;
                }
                return Some(res);
            }
        }

        indexes.0 = (indexes.0 as isize - dirs[dir_index].0) as usize;
        indexes.1 = (indexes.1 as isize - dirs[dir_index].1) as usize;

        dir_index = (dir_index + 1) % dirs.len();
    }
    None
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut possible_positions = Vec::new();

    // Trova tutte le posizioni libere ('.') dove possiamo aggiungere un'ostruzione
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '.' {
                possible_positions.push((i, j));
            }
        }
    }

    // Filtra le posizioni che causano un ciclo
    let mut valid_positions:u32 = 0;

    for &(x, y) in &possible_positions {
        // Aggiungi un'ostruzione temporanea
        matrix[x][y] = '#';

        // Verifica se causa un ciclo
        if causes_cycle(&matrix) {
            valid_positions += 1;
        }

        // Rimuovi l'ostruzione temporanea
        matrix[x][y] = '.';
    }

    Some(valid_positions)
}

fn causes_cycle(matrix: &Vec<Vec<char>>) -> bool {
    let mut indexes = (0, 0);
    let mut direction = (0, -1); // Direzione iniziale: ^ (su)
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // su, destra, giù, sinistra
    let mut visited = HashSet::new();

    // Trova la posizione iniziale della guardia
    for i in 0..matrix.len() {
        if let Some(pos) = matrix[i].iter().position(|&r| r == '^') {
            indexes = (i, pos);
            direction = (-1, 0); // ^ significa su
            break;
        }
    }

    loop {
        // Calcola la prossima posizione
        let next_pos = (
            indexes.0 as isize + direction.0,
            indexes.1 as isize + direction.1,
        );

        // Controlla se siamo fuori dai limiti
        if next_pos.0 < 0
            || next_pos.0 >= matrix.len() as isize
            || next_pos.1 < 0
            || next_pos.1 >= matrix[0].len() as isize
        {
            // Fuori dai limiti: non siamo in un ciclo
            return false;
        }

        // Stato attuale: posizione e direzione
        let state = (indexes, direction);

        // Se lo stato è già stato visitato, c'è un ciclo
        if !visited.insert(state) {
            return true;
        }

        // Se la prossima posizione è un ostacolo, gira a destra
        if matrix[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            let dir_index = dirs.iter().position(|&d| d == direction).unwrap();
            direction = dirs[(dir_index + 1) % dirs.len()];
        } else {
            // Altrimenti, avanza
            indexes = (next_pos.0 as usize, next_pos.1 as usize);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
