use itertools::Itertools;

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub(crate) fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row][col]
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        *self.data.get_mut(row).unwrap().get_mut(col).unwrap() = val;
    }

    pub fn get_neighbors(&self, row: isize, col: isize) -> impl IntoIterator<Item = &T> {
        (row - 1..=row + 1)
            .cartesian_product(col - 1..=col + 1)
            .filter(move |&coord| coord != (row, col))
            .filter(|&(r, c)| {
                (0 <= r) && (r < self.data.len() as isize) && (0 <= c) && (c < self.data[0].len() as isize)
            })
            .map(|(r, c)| &self.data[r as usize][c as usize])
    }
}

impl<T, I1, I2> From<I1> for Grid<T>
where
    I1: IntoIterator<Item = I2>,
    I2: IntoIterator<Item = T>,
{
    fn from(value: I1) -> Self {
        Self {
            data: value.into_iter().map(|row| row.into_iter().collect()).collect(),
        }
    }
}
