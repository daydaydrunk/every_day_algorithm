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
                0 => 0,
                1 => nums[0],
                _ => {
                    let (mut res,mut max,mut min,mut tmax,mut tmin) = (0,0,0,1,1);
                    for n in nums.iter() {
                        tmax *= *n;
                        tmin *= *n;
                        max = max.max(tmax);
                        min = min.min(tmin);

                        res = max.max(min);
                        println!("{:?}",tmax);
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
                    let mut p = 0;
                    let mut s=0;
                    let mut negative = nums[0];
                    for n in nums.iter() {
                        s += *n;
                        s = s.max(0);
                        negative = negative.max(*n);
                        p = p.max(s);
                    }
                    if negative < 0{negative}else{p}
                }
            }
        }

        pub fn insert(nums: Vec<(i32,i32)>) -> Vec<(i32,i32)>{
            let mut ret = Vec::<(i32,i32)>::new();
            let mut tmp = (0,0);
            for (l,r) in nums.into_iter(){
                match tmp{
                    (0,0) => {tmp = (l,r)},
                    _ =>{
                        if l < tmp.0 && r > tmp.1{
                            continue;
                        }else if r > tmp.1 && l-tmp.1 == 1{ // right
                            tmp.1 = r;ret.push(tmp);
                        }else if r < tmp.0 && tmp.0 - r == 1{
                            tmp = (l,r);ret.push(tmp);
                        }else if r > tmp.1 && l-tmp.1 > 1 {
                            tmp = (l,r);ret.push(tmp);
                        }
                    },
                }

            }
            ret
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
    assert_eq!(code::Solution::max_sub_array(vec![-2,-1]),-1);
}

#[test]
fn test_insert(){
    assert_eq!(code::Solution::insert(vec![(1,2),(3,4),(2,3),(7,8),(5,6)]),vec![(1,4),(7,8),(5,6)]);
}
//EOF