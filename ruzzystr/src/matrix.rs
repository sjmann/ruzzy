pub struct Matrix {
    width: usize,
    contents: Vec<usize>,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Matrix {
        Matrix { contents: vec![0; width * height], width}
    }

    pub fn set_element(&mut self, row: usize, col: usize, value: usize) {
        let index = self.width * row + col;
        println!("Index: {} for  row: {} by col: {} || width is {}", index, row, col, self.width);
        println!("{}", self.contents.len());
        self.contents[index] = value;
    }

    pub fn get_element(&self, row: usize, col: usize) -> usize {
        self.contents[self.width * row + col] 
    }
}