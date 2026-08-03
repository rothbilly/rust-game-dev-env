

struct DoubleBuffer<T: Clone>{
    front: Vec<T>,
    back: Vec<T>,
}


impl<T> DoubleBuffer<T> 
where T: Clone 
{
    fn new(initial_state: Vec<T>) -> Self {
        Self {
            // The back buffer starts as a clone of the front
            back: initial_state.clone(),
            front: initial_state,
        }
    }

    fn read(&self) -> &[T] {
        &self.front
    }

    fn write_mut(&mut self) -> &mut [T] {
        &mut self.back
    }

    /// Swap the front and back buffers instantly (O(1) complexity)
    fn swap(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }
}



// Sample Buffer

struct GridBuffer{
    width: usize,
    height: usize,
    df: DoubleBuffer<u8>
}

impl GridBuffer{
    pub fn new() -> GridBuffer{
        GridBuffer{
            width: 0,
            height: 0,
            df: DoubleBuffer::new(Vec::<u8>::new())
        }
    }

    pub fn new_from_size(width: usize, height: usize) -> GridBuffer{
        GridBuffer { 
            width,
            height, 
            df:  DoubleBuffer::new(Vec::<u8>::with_capacity(width * height))}
    }

    #[inline]
    fn index(&self, row: usize, column: usize) -> usize {
        // size is stored as (width, height)
        // validate: row < height, column < width
        if row >= self.width || column >= self.height {
            panic!("the index is greater than the original grid");
        }

        // Core formula: column * width + column
        column * self.width + row
    }

    fn coordinates_from_index(&self, idx: usize) -> (usize, usize){
        let x = idx % self.width;
        let y = idx / self.width; 
        (x,y)
    }

    fn read_at_index(&self, x: usize, y: usize) -> u8 {
        let idx = self.index(x, y);
        let read_bf =  self.df.read();
        read_bf[idx]
    }


    fn write_at_index(&mut self, x: usize, y: usize, value: u8){
        let idx = self.index(x, y);
        let write_bff =  self.df.write_mut();
        write_bff[idx] = value;
    }

}



