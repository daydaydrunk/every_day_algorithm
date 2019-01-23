pub mod code{

    #[derive(Debug)]
    pub struct Solution;

    impl Solution {
        pub fn rob(nums: Vec<i32>) -> i32 {
            let mut p1:i32 = 0;
            let mut p2:i32 = 0;
            for n in nums.iter(){
                let tmp = p1;
                p1 = p1.max(p2 + *n);
                p2 = tmp;
//                println!("{:?}=>{:?} {:?} {:?}",n,tmp,p1,p2);
            }
            p1
        }

        //leetcode 151
        pub fn re_words(s:&str) -> String{
            s.split_whitespace().rev().fold(String::new(),|acc,x| acc + x + " ").trim().to_string()
        }

        pub fn max_product(nums: Vec<i32>) -> i32 {
            let mut res:i32 = 0;
            match nums.len() {
                1 => nums[0],
                _ => {
                    for (i, n) in nums.iter().enumerate() {
                        let p = match nums.get(i + 1) {
                            Some(j) => {
                                if (j - *n).abs() == 1 { j * (*n) } else { 0 }
                            },
                            None => *n,
                        };
                        if p > res { res = p; }
                    }
                    res
                },
            }
        }

        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            match nums.len() {
                0 => 0,
                1 => nums[0],
                _ => {
                    let mut p = nums[0];
                    let mut s=0;
                    for n in nums.iter() {
                        let max = s.max(s+n);
                        p=p.max(s);
                    }
                    p
                }
            }
        }
    }
}

#[test]
fn test_rob(){
//    assert_eq!(code::Solution::rob(vec![2,7,9,3,1]),12);
//    assert_eq!(code::Solution::rob(vec![0]),0);
//    assert_eq!(code::Solution::rob(vec![2,1,1,2]),4);
//    assert_eq!(code::Solution::rob(vec![6,6,4,8,4,3,3,10]),27);
    assert_eq!(code::Solution::rob(vec![2,4,8,9,9,3]),19);
}

#[test]
fn test_re_words(){
    let mut str1 = "this is a   pen  ";
    assert_eq!(code::Solution::re_words(str1),"pen a is this");
}

#[test]
fn test_max_product(){
    assert_eq!(code::Solution::max_product(vec![2,3,0,4]),6);
    assert_eq!(code::Solution::max_product(vec![-2,0,-1]),0);
    assert_eq!(code::Solution::max_product(vec![-2,0,-1,-3,-4]),12);
    assert_eq!(code::Solution::max_product(vec![-2]),-2);
    assert_eq!(code::Solution::max_product(vec![2]),2);
    assert_eq!(code::Solution::max_product(vec![0]),0);
    assert_eq!(code::Solution::max_product(vec![-1,-2,-3,9,-8,-9]),72);
}

#[test]
fn test_max_sub_array(){
    assert_eq!(code::Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]),6);
    assert_eq!(code::Solution::max_sub_array(vec![-1]),-1);
}

//EOF