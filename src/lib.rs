use std::convert::{TryFrom, TryInto};
use std::collections::HashSet;

// Supports replace operations
pub struct Hamming {
    a: String,
    b: String,
}

impl Hamming {
    pub fn new(a: &str, b: &str) -> Hamming {
        Hamming {
            a: a.to_string(),
            b: b.to_string(),
        }
    }

    pub fn cmp(&self) -> u32 {
        let len_a = i32::try_from(self.a.chars().count()).unwrap();
        let len_b = i32::try_from(self.b.chars().count()).unwrap();
        let len_short = if len_a > len_b { len_b } else { len_a };
        let d_len = (len_b - len_a).abs();
        let mut distance = d_len;

        for (idx, a_chr) in self.a.chars().enumerate() {
            if idx < len_short.try_into().unwrap() {
                let b_chr = self.b.chars().nth(idx).unwrap();
                if a_chr != b_chr {
                    distance += 1;
                }
            }
        }

        distance.try_into().unwrap()
    }
}

// Supports insert, delete & replace operations
pub struct Levenshtein {
    a: String,
    b: String,
}

impl Levenshtein {
    pub fn new(a: &str, b: &str) -> Levenshtein {
        Levenshtein {
            a: a.to_string(),
            b: b.to_string(),
        }
    }

    pub fn cmp(&self) -> u32 {
        let len_a: usize = self.a.chars().count();
        let len_b: usize = self.b.chars().count();

        let char_a: Vec<char> = self.a.chars().collect();
        let char_b: Vec<char> = self.b.chars().collect();

        // Matrix implementation
        let mut calc_matrix: Vec<Vec<u32>> = vec![vec![0; len_a+1]; len_b+1];

        // Initialize score matrix
        for i in 0..=len_a {
            calc_matrix[0][i] = i.try_into().unwrap();
        }
        for i in 0..=len_b {
            calc_matrix[i][0] = i.try_into().unwrap();
        }

        let mut tmp: Vec<u32> = vec![];
        for i in 1..=len_b {
            for j in 1..=len_a {
                let char_match = char_a[j-1] == char_b[i-1];

                tmp.push(calc_matrix[i-1][j]+1);
                tmp.push(calc_matrix[i][j-1]+1);
                if char_match { 
                    tmp.push(calc_matrix[i-1][j-1]);
                } else {
                    tmp.push(calc_matrix[i-1][j-1]+1);
                }

                calc_matrix[i][j] = *tmp.iter().min().unwrap();
                tmp.clear()
            }
        }

        calc_matrix[len_b][len_a]
    }

    pub fn dump_matrix(&self, matrix: &Vec<Vec<u32>>) {
        for (i, i_val) in matrix.iter().enumerate() {
            println!("{:?}", i_val);
        }
    }
}

pub struct Jaccard {
    a: String,
    b: String,
}

impl TwoWayCmp for Jaccard {
    fn new(a: &str, b: &str) -> Jaccard {
        Jaccard {
            a: a.to_string(),
            b: b.to_string(),
        }
    }
}

impl Jaccard {
    pub fn cmp(&self) -> f32 {
        let char_a: HashSet<char> = self.a.chars().collect::<HashSet<char>>();
        let char_b: HashSet<char> = self.b.chars().collect::<HashSet<char>>();
    
        // / char_b.union(&char_b).collect().len();
        let hs_a_i_b: HashSet<_> = char_a.intersection(&char_b).collect();
        let hs_a_u_b: HashSet<_> = char_a.union(&char_b).collect();
        let j = hs_a_i_b.len() as f32/hs_a_u_b.len() as f32;
        j
    }
}

pub trait TwoWayCmp {
    fn new(a: &str, b: &str) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical() {
        let h_dist = Hamming::new("abc", "abc").cmp();
        let l_dist = Levenshtein::new("abc", "abc").cmp();
        assert_eq!(h_dist, 0);
        assert_eq!(l_dist, 0);
    }

    #[test]
    fn test_onechar_diff() {
        let h_dist = Hamming::new("abc", "abe").cmp();
        let l_dist = Levenshtein::new("abc", "abe").cmp();
        assert_eq!(h_dist, 1);
        assert_eq!(l_dist, 1);
    }

    #[test]
    fn test_whitespace_diff() {
        let h_dist = Hamming::new("abc ", "abc").cmp();
        let l_dist = Levenshtein::new("abc ", "abc").cmp();
        assert_eq!(h_dist, 1);
        assert_eq!(l_dist, 1);
    }

    #[test]
    fn test_whitespace_char_diff() {
        let h_dist = Hamming::new("abc d", "abc").cmp();
        let l_dist = Levenshtein::new("abc d", "abc").cmp();
        assert_eq!(h_dist, 2);
        assert_eq!(l_dist, 2);
    }

    #[test]
    fn test_whitespace_only() {
        let h_dist = Hamming::new(" ", "").cmp();
        let l_dist = Levenshtein::new(" ", "").cmp();
        assert_eq!(h_dist, 1);
        assert_eq!(l_dist, 1);
    }
}
