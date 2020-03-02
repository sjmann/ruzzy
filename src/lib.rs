use std::cmp;

pub fn levenshtein_distance(a: &str, b:&str) -> u32 {
    let a_len = a.len() + 1;
    let b_len = b.len() + 1;

    let mut matrix: Vec<Vec<usize>> = vec![vec![0; b_len]; a_len];

    for i in 1..a_len {
        matrix[i][0] = i;
    }

    for j in 1..b_len {
        matrix[0][j] = j;
    }

    for j in 1..b_len {
        for i in 1..a_len {
            let substitution_cost = match a.as_bytes()[i -1] == b.as_bytes()[j - 1] {
                true => 0,
                false => 1,
            };

            let deletion = matrix[i-1][j] + 1;
            let insertion = matrix[i][j-1] + 1;
            let substitution = matrix[i-1][j-1] + substitution_cost;
            matrix[i][j] = cmp::min(cmp::min(deletion, insertion), substitution);
        }
    }
    println!("{:?}", matrix);
    matrix[a.len()][b.len()] as u32
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
}