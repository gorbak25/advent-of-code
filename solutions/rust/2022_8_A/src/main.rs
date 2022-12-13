extern crate support;
use std::iter;
use std::cmp;

// Iterator over a given row or column in a 2d vector
struct Vector2DLineIterator<'a, T> {
    // Index of next element to be returned
    pos_start: (usize, usize),
    // Index of one element past the end
    pos_end: (usize, usize),
    step: (usize, usize),
    vec2d: &'a Vec<Vec<T>>,
}

impl<'a, T> Vector2DLineIterator<'a, T> {
    fn into_iter(vec2d: &'a Vec<Vec<T>>, column_mode: bool, row_or_col: usize) -> Vector2DLineIterator<'a, T> {
        let (pos_start, pos_end, step) = if column_mode {
            ((row_or_col, 0), (row_or_col, vec2d.len()), (0, 1))
        } else {
            ((0, row_or_col), (vec2d[row_or_col].len(), row_or_col), (1, 0))
        };
        Vector2DLineIterator {
            pos_start,
            pos_end,
            step,
            vec2d
        }
    } 
}

impl<'a, T> Iterator for Vector2DLineIterator<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos_start == self.pos_end { 
            return None; 
        }
        let r = (self.pos_start, self.vec2d.get(self.pos_start.1)?.get(self.pos_start.0)?);
        self.pos_start = (self.pos_start.0 + self.step.0, self.pos_start.1 + self.step.1);
        Some(r)
    }
}

impl<'a, T> DoubleEndedIterator for Vector2DLineIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.pos_start == self.pos_end { 
            return None; 
        }
        self.pos_end = (self.pos_end.0 - self.step.0, self.pos_end.1 - self.step.1);
        let r = (self.pos_end, self.vec2d.get(self.pos_end.1)?.get(self.pos_end.0)?);
        Some(r)
    }
}

fn main() {
    let grid: Vec<Vec<u8>> = 
        support::test_data!()
        .trim()
        .lines()
        .map(|x| 
            x.trim().chars()
            .map(|b| b as u8 - b'0')
            .collect::<Vec<u8>>()
        )
        .collect::<Vec<Vec<u8>>>();

    // All trees are not visible until proven otherwise
    let mut visibility: Vec<Vec<bool>> =
        vec![vec![false; grid[0].len()]; grid.len()];

    // For each row and column in the grid
    for (colum_mode, row_or_col) in iter::repeat(false).zip(0..grid.len()).chain(iter::repeat(true).zip(0..grid[0].len())) {
        // Iterate that row or colum either forward or backwards
        // TODO: is this possible without boxing or moving the loop logic to a separate function?
        for it in [
            Box::new(Vector2DLineIterator::into_iter(&grid, colum_mode, row_or_col)) as Box<dyn Iterator<Item = _>>, 
            Box::new(Vector2DLineIterator::into_iter(&grid, colum_mode, row_or_col).rev())
            ] {
            let mut prev_max = None;
            for ((x, y), &el) in it {
                if prev_max.is_none() {
                    // Trees on the edge are always visible
                    visibility[y][x] = true;
                    prev_max = Some(el);
                    continue;
                }
                let max = prev_max.unwrap();
                // We are taller that a previous tree so we are visible
                if max < el {
                    visibility[y][x] = true;
                }
                prev_max = Some(cmp::max(el, max));
            }
        }
    };

    println!("{}", visibility.iter().map(|row| row.iter().filter(|&&el| el).count()).sum::<usize>())
}
