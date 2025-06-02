#[derive(Debug, Clone)]
pub struct Solution {
    pub ss: Vec<usize>,
    pub ms: Vec<usize>,
}

impl Solution {
    pub fn new(ss: Vec<usize>, ms: Vec<usize>) -> Self {
        assert_eq!(ss.len(), ms.len(), "ss 和 ms 長度必須相同");
        Solution { ss, ms }
    }
}