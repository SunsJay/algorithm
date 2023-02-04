impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut t = 1;
        let mut ans = digits;
        for i in (0..ans.len()).rev(){
            if ans[i]+t < 10{
                ans[i] += t;
                return ans;
            }else{
                ans[i] = (t+ans[i])%10;
                t = 1;
                if i == 0{
                    ans.insert(0, 1);
                    return ans;
                }
            }
        }
        ans
    }
}



struct Solution {}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        let a = vec![1, 9, 9];
        let b = Solution::plus_one(a);
        println!("{:?}", b);
    }
}