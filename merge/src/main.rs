fn main() {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n != 0 {
            if m == 0 {
                for i in 0..n {
                    nums1[i as usize] = nums2[i as usize]
                }
            }
            else {
                let mut i1: usize = m as usize - 1;
                let mut i2: usize = n as usize - 1;
                let mut done1: bool = false;
                let mut done2: bool = false;

                for i in (0..m+n).rev() {
                    if done1 || (nums2[i2] >= nums1[i1] && !done2) {
                        nums1[i as usize] = nums2[i2];
        
                        if i2 != 0 {
                            i2 -= 1;
                        }
                        else {
                            done2 = true;
                        }
                    }
                    else {
                        nums1[i as usize] = nums1[i1];
                        if i1 != 0 {
                            i1 -= 1;
                        }
                        else {
                            done1 = true;
                        }
                    }
                }
            }
        }
    }

    let mut nums1: Vec<i32> = vec![2,0];
    let mut nums2: Vec<i32> = vec![1];
    let n: i32 = nums2.len() as i32;
    let m: i32 = nums1.len() as i32 - n;


    merge(&mut nums1, m, &mut nums2, n);
    
    println!("{:?}", nums1);

    /*
    #[cfg(test)]
    mod tests {
        assert_eq!(merge([1,2,3,0,0,0], 3, [2,5,6], 3), [1,2,2,3,5,6]);
    } */
}
