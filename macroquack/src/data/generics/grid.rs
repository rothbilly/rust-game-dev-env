#[derive(Clone, Debug)]
struct VecGrid<T: Clone> {
    width: usize,
    height: usize,
    items: Vec<T>,
}

impl<T> VecGrid<T>
where
    T: Clone + Default,
{
    pub fn new() -> VecGrid<T> {
        VecGrid {
            width: 0,
            height: 0,
            items: Vec::<T>::new(),
        }
    }

    pub fn new_from_size(width: usize, height: usize) -> VecGrid<T> {
        VecGrid {
            width,
            height,
            items: vec![T::default(); width * height],
        }
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        // size is stored as (width, height)
        // validate: row < height, column < width
        if x >= self.width || y >= self.height {
            panic!("the index is greater than the original grid");
        }

        // Core formula: column * width + row
        y * self.width + x
    }

    fn coordinates_from_index(&self, idx: usize) -> (usize, usize) {
        let x = idx % self.width;
        let y = idx / self.width;
        (x, y)
    }

    fn read_at_index(&self, x: usize, y: usize) -> &T {
        let idx = self.index(x, y);
        &self.items[idx]
    }

    fn write_at_index(&mut self, x: usize, y: usize, value: T) {
        let idx = self.index(x, y);
        self.items[idx] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn grid_size_testing() {
        let grid_1 = VecGrid::<u8>::new_from_size(10, 10);
        println!("grid {:?}", &grid_1);
        assert_eq!(grid_1.items.len(), 100)
    }

    #[test]
    pub fn grid_index_testing() {
        let rect_size: usize = 10;

        let mut grid_1 = VecGrid::<u8>::new_from_size(rect_size, rect_size);
        let mut counter: usize = 0;
        for item in grid_1.items.iter_mut() {
            *item = counter.clone() as u8;
            counter += 1;
        }
        println!("grid {:?}", &grid_1);
    }
}
