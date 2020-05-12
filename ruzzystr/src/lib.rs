use std::cmp;
use std::convert::TryInto;

struct Matrix {
    contents: Vec<Vec<usize>>
}

impl Matrix {
    fn new(x_len: usize, y_len: usize) -> Matrix {
        Matrix { contents: vec![vec![0; y_len]; x_len]}
    }

    fn set_at(&mut self, x: usize, y: usize, value: usize) {
        self.contents[x][y] = value;
    }

    fn get_at(&self, x: usize, y: usize) -> usize {
        self.contents[x][y]
    }
}

pub fn levenshtein_distance(a: &str, b:&str) -> u32 {
    let matrix_x_len = a.len() + 1;
    let matrix_y_len = b.len() + 1;

    let mut matrix = Matrix::new(matrix_x_len, matrix_y_len);

    for i in 1..matrix_x_len {
        matrix.set_at(i, 0, i);
    }

    for j in 1..matrix_y_len {
        matrix.set_at(0, j, j);
    }

    for j in 1..matrix_y_len {
        for i in 1..matrix_x_len {
            let substitution_cost = usize::from(a.as_bytes()[i - 1] != b.as_bytes()[j - 1]);
            let deletion = matrix.get_at(i - 1, j) + 1;
            let insertion = matrix.get_at(i, j - 1) + 1;
            let substitution = matrix.get_at(i - 1, j - 1) + substitution_cost;
            matrix.set_at(i, j, cmp::min(cmp::min(deletion, insertion), substitution));
        }
    }
    matrix.get_at(a.len(), b.len()) as u32
}

pub fn longest_common_substring<'a>(str_a: &str, str_b: &str) -> &'a str {
    let matrix_x_len = str_a.len() + 1;
    let matrix_y_len = str_b.len() + 1;

    let mut matrix = Matrix::new(matrix_x_len, matrix_y_len);

    for i in 0..matrix_x_len - 1 {
        matrix.set_at(0, i, 0);
    }

    for j in 0..matrix_y_len - 1 {
        matrix.set_at(j, 0, 0);
    }

    let str_a_bytes = str_a.as_bytes();
    let str_b_bytes = str_b.as_bytes();
    for j in 1..matrix_y_len - 1 {
        for i in 1..matrix_x_len - 1 {
            if str_a_bytes[i - 1] == str_b_bytes[j - 1] {
                matrix.set_at(i, j, matrix.get_at(i - 1, j - 1) + 1);
            } else {
                matrix.set_at(i, j, 0);
            }
        }
    }

    let result: i32 = -1;

    for j in 1..matrix_y_len - 1 {
        for i in 1..matrix_x_len - 1 {
            if result < matrix.get_at(i, j).try_into().unwrap() {
                result = matrix.get_at(i, j).try_into().unwrap();
            }
        }
    }

    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levenshtein_distance_puppy_guppy() {
        assert_eq!(levenshtein_distance("puppy", "guppy"), 1);
    }

    #[test]
    fn levenshtein_distance_kitten_sitting() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn longest_common_substring_puppy_guppy() {
        assert_eq!(longest_common_substring("puppy", "guppy"), "uppy");
    }
}