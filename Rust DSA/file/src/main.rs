
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 { return 0 }
        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            // println!("i = {:#?}, j = {:#?}", i, j);
            // println!("nums = {:#?}", nums);
            if nums[i] == val {
                let tmp = nums[i];
                nums[i] = nums[j];
                nums[j] = tmp;
                j -= 1;
            } else {
                i += 1;
            }
        }
        (if nums[i] == val { i } else { i + 1 })
        as i32
    }
}