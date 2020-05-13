mod matrix;
use std::cmp;
use std::convert::TryInto;
use matrix::Matrix;

pub fn levenshtein_distance(a: &str, b:&str) -> u32 {
    let matrix_x_len = a.len() + 1;
    let matrix_y_len = b.len() + 1;

    let mut matrix = Matrix::new(matrix_x_len, matrix_y_len);

    for i in 1..matrix_x_len {
        matrix.set_element(i, 0, i);
    }

    for j in 1..matrix_y_len {
        matrix.set_element(0, j, j);
    }

    for j in 1..matrix_y_len {
        for i in 1..matrix_x_len {
            let substitution_cost = usize::from(a.as_bytes()[i - 1] != b.as_bytes()[j - 1]);
            let deletion = matrix.get_element(i - 1, j) + 1;
            let insertion = matrix.get_element(i, j - 1) + 1;
            let substitution = matrix.get_element(i - 1, j - 1) + substitution_cost;
            matrix.set_element(i, j, cmp::min(cmp::min(deletion, insertion), substitution));
        }
    }
    matrix.get_element(a.len(), b.len()) as u32
}

pub fn longest_common_substring(str_a: &str, str_b: &str) -> u32 {
    let matrix_x_len = str_a.len() + 1;
    let matrix_y_len = str_b.len() + 1;

    let mut matrix = Matrix::new(matrix_x_len, matrix_y_len);

    for i in 0..matrix_x_len - 1 {
        matrix.set_element(0, i, 0);
    }

    for j in 0..matrix_y_len - 1 {
        matrix.set_element(j, 0, 0);
    }

    let str_a_bytes = str_a.as_bytes();
    let str_b_bytes = str_b.as_bytes();
    for j in 1..matrix_y_len {
        for i in 1..matrix_x_len {
            if str_a_bytes[i - 1] == str_b_bytes[j - 1] {
                matrix.set_element(i, j, matrix.get_element(i - 1, j - 1) + 1);
            } else {
                matrix.set_element(i, j, 0);
            }
        }
    }

    let mut result: i32 = -1;

    for j in 1..matrix_y_len {
        for i in 1..matrix_x_len {
            if result < matrix.get_element(i, j).try_into().unwrap() {
                result = matrix.get_element(i, j).try_into().unwrap();
            }
        }
    }
    result.try_into().unwrap()    
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
        assert_eq!(longest_common_substring("puppy", "guppy"), 4);
    }

    #[test]
    fn longest_common_substring_kitten_sitting() {
        assert_eq!(longest_common_substring("kitten", "sitting"), 3);
    }
}