/*
Given two numbers, n and k, make all unique combinations of k numbers from 1 to n and in sorted order
*/

pub struct Combine {
    combinations: Vec<Vec<i32>>,
    current: Vec<i32>,
    n: i32,
    k: i32,
}

impl Combine {
    fn new(n: i32, k: i32) -> Self {
        Combine {
            combinations: Vec::new(),
            current: Vec::new(),
            n,
            k,
        }
    }

    fn find_combination(&mut self, _start: Option<i32>) -> Vec<Vec<i32>> {
        let start = _start.unwrap_or(1);

        if self.current.len() == self.k as usize {
            self.combinations.push(self.current.clone());
        } else {
            for i in start..=self.n {
                self.current.push(i);
                self.find_combination(Some(i + 1));
                self.current.pop();
            }
        }

        self.combinations.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Combine;

    #[test]
    fn combinations_3_2() {
        assert_eq!(
            Combine::new(3, 2).find_combination(None),
            vec![[1, 2], [1, 3], [2, 3]]
        );
    }

    #[test]
    fn combination_4_2() {
        assert_eq!(
            Combine::new(4, 2).find_combination(None),
            vec![[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
        );
    }
}
