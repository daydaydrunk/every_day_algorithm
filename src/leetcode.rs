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
            "".to_string()
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
    println!("{:?}",code::Solution::re_words(str1));



//    for c in str1.split_whitespace().collect::<Vec<&str>>() {
//        s1.push(c);
//        println!("{:?}",c);
//    }
    let mut s1 = vec!["a","b","c"];
    println!("{:?} {:?}",s1.into_iter().map(| x| { x.to_owned() + "c"}).collect::<Vec<String>>(),1);
    assert_eq!(str1,"pen a is this");
}

//EOF