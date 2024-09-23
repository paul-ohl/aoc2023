pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<char>>,
}

impl From<&str> for Matrix {
    fn from(s: &str) -> Self {
        let data: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let width = data[0].len();
        let height = data.len();
        Matrix {
            width,
            height,
            data,
        }
    }
}

impl From<String> for Matrix {
    fn from(s: String) -> Self {
        Matrix::from(s.as_str())
    }
}

impl Matrix {
    /// Read a character from the matrix at the given coordinates.
    pub fn read_coord(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }

    /// Find all coordinates of a given character in the matrix.
    pub fn find_all_coords(&self, c: char) -> Vec<(usize, usize)> {
        let mut coords = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.read_coord(x, y) == c {
                    coords.push((x, y));
                }
            }
        }
        coords
    }

    /// Read a number from the matrix at the given coordinates.
    ///
    /// It doesn't matter which digit of the number is pointed to by the coordinates, the whole number will be returned.
    pub fn number_from_coord(&self, x: usize, y: usize) -> i32 {
        if !self.read_coord(x, y).is_numeric() {
            return 0;
        }
        let line = &self.data[y];

        let num_start: String = line
            .iter()
            .rev()
            .skip(self.width - x)
            .take_while(|c| c.is_numeric())
            .collect();
        let num_end: String = line.iter().skip(x).take_while(|c| c.is_numeric()).collect();

        let num = format!("{}{}", num_start.chars().rev().collect::<String>(), num_end);
        num.parse().unwrap()
    }

    /// Get all adjacent coordinates that satisfy a given condition.
    ///
    /// The condition is a function that takes a character and returns a boolean.
    /// The function will not return the coordinates of the cell itself, even if it satisfies the condition.
    pub fn get_adjacent_if(
        &self,
        x: usize,
        y: usize,
        condition: fn(char) -> bool,
    ) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();

        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                let x = x as i32 + x_offset;
                let y = y as i32 + y_offset;
                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                    continue;
                }
                if condition(self.read_coord(x as usize, y as usize)) {
                    adjacent.push((x as usize, y as usize));
                }
            }
        }
        adjacent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_number_from_coord() {
        let matrix = Matrix::from("..123\n456..\n.7.91\n43217");
        assert_eq!(matrix.number_from_coord(0, 0), 0);
        assert_eq!(matrix.number_from_coord(2, 0), 123);
        assert_eq!(matrix.number_from_coord(3, 0), 123);
        assert_eq!(matrix.number_from_coord(4, 0), 123);
        assert_eq!(matrix.number_from_coord(0, 1), 456);
        assert_eq!(matrix.number_from_coord(2, 1), 456);
        assert_eq!(matrix.number_from_coord(1, 2), 7);
        assert_eq!(matrix.number_from_coord(3, 2), 91);
        assert_eq!(matrix.number_from_coord(4, 2), 91);
        assert_eq!(matrix.number_from_coord(0, 3), 43217);
        assert_eq!(matrix.number_from_coord(4, 3), 43217);
    }

    #[test]
    fn test_get_adjacent_if() {
        let matrix = Matrix::from("##..\n#...\n....\n....");
        let adjacent = matrix.get_adjacent_if(0, 0, |c| c == '#');
        assert_eq!(adjacent, vec![(1, 0), (0, 1)]);
        let adjacent = matrix.get_adjacent_if(1, 1, |c| c == '#');
        assert_eq!(adjacent, vec![(0, 0), (1, 0), (0, 1)]);
        let adjacent = matrix.get_adjacent_if(2, 2, |c| c == '#');
        assert_eq!(adjacent, vec![]);
        let adjacent = matrix.get_adjacent_if(2, 2, |c| c == '.');
        assert_eq!(
            adjacent,
            vec![
                (1, 1),
                (2, 1),
                (3, 1),
                (1, 2),
                (3, 2),
                (1, 3),
                (2, 3),
                (3, 3)
            ]
        );
    }
}
