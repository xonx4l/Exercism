pub fn annotate(field: &[&str]) -> Vec<String> {
    let height = field.len();
    let width = if height > 0 { field[0].len() } else { 0 };
    let mut result = vec![vec![' '; width]; height];

    // Helper function to count mines in neighboring cells
    fn count_neighboring_mines(field: &[&str], row: i32, col: i32) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;  // Skip the cell itself
                }
                let ny = row + dy;
                let nx = col + dx;
                if ny >= 0 && ny < field.len() as i32 && nx >= 0 && nx < field[0].len() as i32 {
                    if field[ny as usize].as_bytes()[nx as usize] == b'*' {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    // Iterate through each cell in the field
    for (row, line) in field.iter().enumerate() {
        for (col, &cell) in line.as_bytes().iter().enumerate() {
            if cell == b'*' {
                result[row][col] = '*';
            } else {
                let count = count_neighboring_mines(field, row as i32, col as i32);
                result[row][col] = if count == 0 { ' ' } else { (count + b'0') as char };
            }
        }
    }

    // Convert the 2D vector of chars to a vector of strings
    result.into_iter().map(|row| row.into_iter().collect()).collect()
}
