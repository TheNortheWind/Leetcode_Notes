fn main() {
    struct Solution {
        nums1: Vec<i32>,
        nums2: Vec<i32>
    }

    impl Solution {
        pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
            let mut limbo: Vec<i32> = Vec::new();
            for i in 0..nums1.len() {
                let i: usize = i.try_into().unwrap();
                for k in 0..nums2.len() {
                    let k: usize = k.try_into().unwrap();
                    if nums1[i] == nums2[k] {
                        limbo.push(nums2[k]);
                        limbo.push(nums1[i]);
                    }
                }
            }
            for i in 0..limbo.len() { //insertion sort
                let mut k = i;
                for j in (i+1)..limbo.len() {
                    if limbo[k] > limbo[j] {
                        k = j;
                    }
                }
                limbo.swap(i,k);
            }
            limbo[0] as i32
        }
    }

    /*let z = Solution::get_common([3,2,5,1,2,3].to_vec(),[2,4,3,1,5,2].to_vec());
    println!("{z}");*/
}
