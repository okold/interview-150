fn main() {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        fn get_indices(n: usize, index: usize) -> (usize, usize) {
            return (index / n, index % n);
        }

        let m = matrix.len();
        let n = matrix[0].len();

        let mut left: i32 = 0;
        let mut right: i32 = (m as i32 * n as i32) - 1;
        
        loop {
            if left > right {
                return false;
            }

            let mid = (left + right) / 2;
            let (m_prime, n_prime) = get_indices(n, mid as usize);

            if matrix[m_prime][n_prime] == target {
                return true;
            }
            else if matrix[m_prime][n_prime] < target {
                left = mid + 1;
            }
            else {
                right = mid - 1;
            }
        }     
    }

    assert_eq!(search_matrix([[0,1,2,3].to_vec(),[4,5,6,7].to_vec(),[8,9,10,11].to_vec()].to_vec(), 4), true);
    assert_eq!(search_matrix([[1,3,5,7].to_vec(),[10,11,16,20].to_vec(),[23,30,34,60].to_vec()].to_vec(), 3), true);
    assert_eq!(search_matrix([[1].to_vec()].to_vec(), 0), false);
}
