/*
 * Problem Statement: Generate all distinct permutations of a an array (all permutations should be in sorted order);
 *
 * What is permutations?
 * - Permutation means possible arrangements in a set (here it is an array);
 *
 * Reference to know more about permutations:
 * - https://www.britannica.com/science/permutation
 *
 */

pub struct Permutation {
    pub permutations: Vec<Vec<i32>>,
    pub high: i32,
    pub vec: Vec<i32>,
}

impl Permutation {
    pub fn new(vec: Vec<i32>) -> Self {
        Permutation {
            permutations: vec![],
            high: vec.len() as i32 - 1,
            vec,
        }
    }

    pub fn find_permutations(
        &mut self,
        _low: Option<i32>,
        _vec: Option<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut vec = _vec.unwrap_or(self.vec.clone());
        let low = _low.unwrap_or(0);

        if low == self.high {
            self.permutations.push(vec.clone());
        } else {
            for i in low..=self.high {
                vec.swap(low as usize, i as usize);
                self.find_permutations(Some(low + 1), Some(vec.clone()));
            }
        };

        self.permutations.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Permutation;

    #[test]
    fn should_work() {
        assert_eq!(
            Permutation::new(vec![1, 2, 3]).find_permutations(None, None),
            vec![
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        );
    }
}
