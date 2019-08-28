use crate::leetcode::code::Solution;
use std::cmp::max;
use std::convert::TryInto;
pub mod code {
    use std::collections::hash_set::HashSet;
    use std::ops::Index;

    #[derive(Debug)]
    pub struct Solution;

    impl Solution {
        pub fn rob(nums: Vec<i32>) -> i32 {
            let mut p1: i32 = 0;
            let mut p2: i32 = 0;
            for n in nums.iter() {
                let tmp = p1;
                p1 = p1.max(p2 + *n);
                p2 = tmp;
                //                println!("{:?}=>{:?} {:?} {:?}",n,tmp,p1,p2);
            }
            p1
        }

        //leetcode 151
        pub fn re_words(s: &str) -> String {
            s.split_whitespace()
                .rev()
                .fold(String::new(), |acc, x| acc + x + " ")
                .trim()
                .to_string()
        }

        pub fn max_product(nums: Vec<i32>) -> i32 {
            let mut res: i32 = 0;
            match nums.len() {
                0 => 0,
                1 => nums[0],
                _ => {
                    let (mut res, mut max, mut min, mut tmax, mut tmin) = (0, 0, 0, 1, 1);
                    for n in nums.iter() {
                        tmax *= *n;
                        tmin *= *n;
                        max = max.max(tmax);
                        min = min.min(tmin);

                        res = max.max(min);
                        println!("{:?}", tmax);
                    }
                    res
                }
            }
        }

        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            match nums.len() {
                0 => 0,
                1 => nums[0],
                _ => {
                    let mut p = 0;
                    let mut s = 0;
                    let mut negative = nums[0];
                    for n in nums.iter() {
                        s += *n;
                        s = s.max(0);
                        negative = negative.max(*n);
                        p = p.max(s);
                    }
                    if negative < 0 {
                        negative
                    } else {
                        p
                    }
                }
            }
        }

        pub fn insert(nums: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
            let mut ret = Vec::<(i32, i32)>::new();
            let mut tmp = (0, 0);
            for (l, r) in nums.into_iter() {
                match tmp {
                    (0, 0) => tmp = (l, r),
                    _ => {
                        if l < tmp.0 && r > tmp.1 {
                            continue;
                        } else if r > tmp.1 && l - tmp.1 == 1 {
                            // right
                            tmp.1 = r;
                            ret.push(tmp);
                        } else if r < tmp.0 && tmp.0 - r == 1 {
                            tmp = (l, r);
                            ret.push(tmp);
                        } else if r > tmp.1 && l - tmp.1 > 1 {
                            tmp = (l, r);
                            ret.push(tmp);
                        }
                    }
                }
            }
            ret
        }

        // leetcode 14
        pub fn longest_common_prefix(strs: Vec<String>) -> String {
            if strs.len() == 0 {
                {
                    return "".to_string();
                }
            }
            let mut result: Vec<u8> = Vec::new();
            let mut idx = 0;
            loop {
                let mut last = 0;
                for str in strs.iter() {
                    if str.len() == 0 || idx >= str.len() {
                        return String::from_utf8(result).unwrap();
                    }
                    let arr = str.as_bytes();
                    if last == 0 {
                        last = arr[idx] as u8;
                        continue;
                    }
                    if last != arr[idx] {
                        return String::from_utf8(result).unwrap();
                    }
                }
                result.push(last);
                last = 0;
                idx += 1;
            }
            String::from_utf8(result).unwrap()
        }

        //        func intToRoman(num int) string {
        //        m := []string{"", "M", "MM", "MMM"}
        //        c := []string{"", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"}
        //        x := []string{"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"}
        //        i := []string{"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"}
        //        return m[num/1000] + c[(num%1000)/100] + x[(num%100)/10] + i[num%10]
        //        }

        // LeetCode 13
        pub fn roman_to_int(s: String) -> i32 {
            if s.len() == 0 {
                return 0;
            }
            let ps = s.as_str();
            let m: Vec<&str> = vec!["", "M", "MM", "MMM"];
            let c: Vec<&str> = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
            let x: Vec<&str> = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
            let i: Vec<&str> = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
            let (mut ret, mut l, mut r) = (0, 0, 1);

            let x1 = Solution::get_range(&m, ps, &mut l, &mut r) * 1000;
            let x2 = Solution::get_range(&c, ps, &mut l, &mut r) * 100;
            let x3 = Solution::get_range(&x, ps, &mut l, &mut r) * 10;
            let x4 = Solution::get_range(&i, ps, &mut l, &mut r);
            return x1 + x2 + x3 + x4;
        }

        fn get_range(v: &Vec<&str>, s: &str, l: &mut usize, r: &mut usize) -> i32 {
            let mut f = true;
            while f {
                let ss = match s.get(*l..*r) {
                    Some(t) => t,
                    None => "",
                };
                if !v.contains(&ss) {
                    f = false;
                    *r -= 1;
                    break;
                }
                if *r + 1 > s.len() {
                    break;
                } else {
                    *r += 1;
                }
            }

            match v
                .iter()
                .enumerate()
                .find(|(_, &x)| x == s.get(*l..*r).unwrap())
            {
                Some((i, _)) => {
                    *l = *r;
                    if *r + 1 < s.len() {
                        *r += 1
                    } else {
                        *r = s.len()
                    };
                    i as i32
                }
                None => 0,
            }
        }

        // leetcode 12
        pub fn int_to_roman(num: usize) -> String {
            let m: Vec<&str> = vec!["", "M", "MM", "MMM"];
            let c: Vec<&str> = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
            let x: Vec<&str> = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
            let i: Vec<&str> = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
            return m[num / 1000].to_owned()
                + c[(num % 1000) / 100]
                + x[(num % 100) / 10]
                + i[(num % 10)];
        }

        //leetcode 20
        pub fn is_valid(s: String) -> bool {
            if s.len() == 1 {
                return false;
            }
            let ls = vec!['{', '(', '['];
            let rs = vec!['}', ')', ']'];
            let mut stk = Vec::new();
            for next_char in s.chars() {
                if next_char.is_whitespace() {
                    return false;
                }
                let (l, ok) = match ls.iter().enumerate().find(|(i, &c)| c.eq(&next_char)) {
                    Some((idx, _)) => (idx, true),
                    None => (0, false),
                };
                if ok {
                    stk.push(l);
                } else {
                    let last = match stk.pop() {
                        Some(l) => l,
                        None => return false,
                    };
                    if !next_char.eq(&rs[last]) {
                        return false;
                    }
                }
            }
            if stk.len() != 0 {
                return false;
            }
            return true;
        }

        //leetcode 22
        //class Solution(object):
        //    def generateParenthesis(self, N):
        //        if N == 0: return ['']
        //        ans = []
        //        for c in xrange(N):
        //            for left in self.generateParenthesis(c):
        //                for right in self.generateParenthesis(N-1-c):
        //                    ans.append('({}){}'.format(left, right))
        //        return ans
        pub fn generate_parenthesis(n: i32) -> Vec<String> {
            let mut ret = Vec::new();
            if n == 0 {
                return { ret };
            }
            let mut path = String::new();
            Solution::gp_dfs(&mut ret, n, n, path);
            ret
        }

        fn gp_dfs(res: &mut Vec<String>, l: i32, r: i32, path: String) {
            if l == 0 && r == 0 {
                res.push(path.to_string());
                return;
            }
            if l > 0 {
                Solution::gp_dfs(res, l - 1, r, path.to_owned() + "(");
            }
            if l < r {
                Solution::gp_dfs(res, l, r - 1, path + ")");
            }
        }

        //leetcode 26
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            0
        }

        pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
            if nums.len() == 0 {
                return 0;
            }
            let mut i = 1;

            for k in 1..nums.len() {
                if nums[k] != nums[k - 1] {
                    nums[i] = nums[k];
                    i += 1;
                }
            }
            i as i32
        }

        //leetcode 27
        pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
            if nums.len() == 0 {
                return 0;
            }
            let mut l = 0;
            let mut offset = 0;
            for i in 0..nums.len() {
                if nums[i - offset] == val {
                    nums.remove(i - offset);
                    offset += 1;
                } else {
                    l += 1;
                }
            }
            l
        }

        //leetcode 28
        pub fn str_str(haystack: String, needle: String) -> i32 {
            match (haystack.len(), needle.len()) {
                (0, 0) => return 0,
                (0, _) => return -1,
                (_, 0) => return 0,
                _ => -1,
            };
            for i in 0..haystack.len() {
                if haystack.as_bytes()[i..].starts_with(needle.as_bytes()) {
                    return i as i32;
                }
            }
            -1
        }

        //leetcode 35
        pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
            let mut vidx = 0;
            for i in 0..nums.len() {
                match (nums[i], target) {
                    (a, b) if a == b => return i as i32,
                    (a, b) if a < b => vidx += 1,
                    _ => (),
                }
            }
            vidx
        }

        //leetcode 38
        pub fn count_and_say(n: i32) -> String {
            unimplemented!()
        }

        //leetcode 58
        pub fn length_of_last_word(s: String) -> i32 {
            let mut idx = 0;
            for c in s.trim_end().as_bytes() {
                idx += 1;
                if c.is_ascii_whitespace() {
                    idx = 0;
                }
            }
            idx
        }

        //leetcode 66
        pub fn plus_one2(digits: Vec<i32>) -> Vec<i32> {
            let mut ret = digits;
            if ret.iter_mut().rev().all(|x| match *x {
                9 => {
                    *x = 0;
                    true
                }
                _ if *x < 9 => {
                    *x += 1;
                    false
                }
                _ => false,
            }) {
                ret.insert(0, 1);
            }
            ret
        }

        pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
            let mut d = digits;
            for i in (0..d.len()).rev() {
                if d[i] == 9 {
                    d[i] = 0;
                } else {
                    d[i] += 1;
                    return d;
                }
            }
            d.insert(0, 1);
            d
        }

        //leetcode 67
        pub fn add_binary(a: String, b: String) -> String {
            let x = i64::from_str_radix(&a, 2).unwrap();
            let y = i64::from_str_radix(&b, 2).unwrap();
            format!("{:b}", x & y)
        }

        //leetcode 69
        // Q&A: what is the differcent between
        // x:f64
        // 1. std::mem::transmute::<f64, i64>(x)
        // 2. x as i64
        pub fn my_sqrt(x: i32) -> i32 {
            let mut n = x as f64;
            let half = 0.5 * n;
            let mut i = unsafe { std::mem::transmute::<f64, i64>(n) }; // why use "transmute"?
            i = 0x5fe6ec85e7de30da - (i >> 1);
            n = unsafe { std::mem::transmute::<i64, f64>(i) };
            for i in 0..3 {
                n = n * (1.5 - half * n * n);
            }
            return (1.0 / n) as i32;
        }

        //leetcode 88
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let mut arr = Vec::new();
            nums2.reverse();
            for i1 in 0..(m + n) as usize {
                if i1 >= m as usize {
                    arr.push(match nums2.pop() {
                        Some(a) => a,
                        None => 0,
                    });
                } else {
                    arr.push(nums1[i1]);
                }
            }
            arr.sort();
            if arr.len() < nums1.len() {
                arr.append(&mut vec![0; nums1.len() - arr.len()]);
            }
            *nums1 = arr;
        }

        //leetcode 118
        pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
            let mut ret: Vec<Vec<i32>> = vec![vec![1]];
            //            match num_rows {
            //                0 => return vec![],
            //                1 => return ret,
            //                _ => (),
            //            };
            //            for l in 1..num_rows {
            //                let mut item: Vec<i32> = vec![1];
            //                for i in 1..l {
            //                    let (a1, a2) = (
            //                        ret[l as usize - 1].get(i as usize - 1),
            //                        ret[l as usize - 1].get(i as usize),
            //                    );
            //                    match (a1, a2) {
            //                        (Some(x), Some(y)) => item.push(*x + *y),
            //                        _ => (),
            //                    }
            //                }
            //                item.push(1);
            //                ret.push(item);
            //            }
            ret
        }

        //leetcode 119
        pub fn get_row(row_index: i32) -> Vec<i32> {
            match row_index {
                0 => return vec![],
                1 => return vec![1],
                _ => (),
            };
            let mut ret: Vec<Vec<i32>> = vec![vec![1]];
            for l in 1..row_index + 1 {
                let mut item: Vec<i32> = vec![1];
                for i in 1..l {
                    let (a1, a2) = (
                        ret[l as usize - 1].get(i as usize - 1),
                        ret[l as usize - 1].get(i as usize),
                    );
                    match (a1, a2) {
                        (Some(x), Some(y)) => item.push(*x + *y),
                        _ => (),
                    }
                }
                item.push(1);
                ret.push(item);
            }
            ret[row_index as usize].to_owned()
        }

        //leetcode 167
        pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
            let mut sum = 0;
            let (mut l, mut r) = (0, numbers.len() - 1);
            while l < r {
                sum = numbers[l] + numbers[r];
                if sum < target {
                    l += 1;
                } else if sum > target {
                    r -= 1;
                } else if sum == target {
                    return vec![(l + 1) as i32, (r + 1) as i32];
                }
            }
            vec![]
        }

        //leetcode 169
        //Boyer-Moore majority vote algorithm
        pub fn majority_element(nums: Vec<i32>) -> i32 {
            if nums.len() == 0 {
                return -1;
            }
            let (mut m, mut c) = (nums[0], 0);
            for i in 0..nums.len() {
                if c == 0 {
                    m = nums[i];
                    c += 1;
                } else if m == nums[i] {
                    c += 1;
                } else {
                    c -= 1;
                }
            }
            if nums.iter().fold(0, |acc, &x| {
                if x == m {
                    return acc + 1;
                }
                acc
            }) > (nums.len()) / 2
            {
                m
            } else {
                -1
            }
        }

        //leetcode 189
        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            if k == 0 {
                return;
            }
            for i in 0..k {
                let a = nums.pop().unwrap();
                nums.insert(0, a);
            }
        }

        //leetcode 217
        pub fn contains_duplicate(nums: Vec<i32>) -> bool {
            if nums.len() == 0 {
                return false;
            }
            let mut n = nums;
            n.sort();
            let mut a = n[0];
            for i in 1..n.len() {
                if n[i] == a {
                    return true;
                }
                a = n[i];
            }
            return false;
        }

        //leetcode 219
        pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
            if nums.len() == 0 {
                return false;
            }
            for i in 0..nums.len() {
                for j in i + 1..nums.len() {
                    let abs = i as i32 - j as i32;
                    if abs.abs() <= k && nums[i] == nums[j] {
                        return true;
                    }
                }
            }
            return false;
        }

        //leetcode 268
        pub fn missing_number(nums: Vec<i32>) -> i32 {
            let mut res = 0;
            for i in 0..nums.len() {
                res ^= (i + 1) as i32 ^ nums[i];
                //res i n
                //0 1 3 -> 2
                //2 2 0 -> 0
                //0 3 1 -> 2
            }
            return res;
        }

        //leetcode 283
        pub fn move_zeroes(nums: &mut Vec<i32>) {
            let l = nums.len();
            let mut idx = 0;
            for i in 0..l {
                if nums[idx] == 0 {
                    let d = nums.remove(idx);
                    nums.insert(l - 1, d);
                    continue;
                }
                idx += 1;
            }
        }

        //leetcode 228
        pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
            let mut range = Vec::<String>::new();
            if nums.len() == 0 {
                return range;
            }
            let (mut s, mut e) = (0, 0);
            for i in 1..nums.len() {
                if nums[i - 1] + 1 != nums[i] {
                    Solution::push_to_range(&s, &e, &mut range, &nums);
                    s = i;
                    e = i;
                } else {
                    e = i;
                }
            }
            Solution::push_to_range(&s, &e, &mut range, &nums);
            range
        }

        fn push_to_range(l: &usize, r: &usize, range: &mut Vec<String>, nums: &Vec<i32>) {
            if *l == *r {
                range.push(format!("{}", nums[*r]));
            } else {
                range.push(format!("{}->{}", nums[*l], nums[*r]));
            }
        }

        //leetcode 414
        pub fn third_max(nums: Vec<i32>) -> i32 {
            let mut snums = nums.clone();
            snums.sort();
            snums.dedup_by(|a, b| a == b);
            let l = snums.len();
            match l {
                0 => 0,
                _ if l < 3 => snums.pop().unwrap(),
                _ if l >= 3 => {
                    snums.reverse();
                    snums[2]
                }
                _ => 0,
            }
        }

        // leetcode 448
        pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
            let mut nums = nums;
            for n in 0..nums.len() {
                let x = nums[n].abs() - 1;
                if nums[x as usize] > 0 {
                    nums[x as usize] *= -1;
                }
            }

            let mut rtn = vec![];
            for (idx, &n) in nums.iter().enumerate() {
                if n > 0 {
                    rtn.push((idx + 1) as i32);
                }
            }
            return rtn;
        }

        //leetcode 485
        pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
            let mut max = 0;
            let mut count = 0;
            for i in 0..nums.len() {
                if nums[i] == 1 {
                    count += 1;
                    max = max.max(count);
                } else {
                    count = 0;
                }
            }
            max
        }

        //leetcode 532
        pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
            if k < 0 {
                return 0;
            }
            let mut sum = 0;
            let mut h: HashSet<i32> = HashSet::new();
            let mut r: HashSet<i32> = HashSet::new();

            for i in 0..nums.len() {
                if h.contains(&nums[i]) {
                    r.insert(nums[i]);
                } else {
                    h.insert(nums[i]);
                }
            }

            for v in h.iter() {
                if h.contains(&(v + k)) {
                    sum += 1;
                }
            }

            if k == 0 {
                return r.len() as i32;
            }

            sum
        }

        //leetcode 561
        pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
            let mut nums = nums;
            nums.sort();
            let mut sum = 0;
            nums.iter().enumerate().fold(0, |acc, (i, x)| {
                if i & 1 == 1 {
                    sum += acc.min(*x);
                }
                *x
            });
            sum
        }

        // leetcode 566
        pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
            let mut ret: Vec<Vec<i32>> = Vec::new();
            let mut ll: Vec<i32> = Vec::new();
            for i in nums.iter() {
                for j in i.iter() {
                    ll.push(*j);
                }
            }

            for rr in 0..r {
                let mut col = Vec::new();
                for cc in 0..c {
                    if let Some(item) = ll.pop() {
                        col.push(item)
                    } else {
                        return nums;
                    }
                }
                col.reverse();
                ret.push(col);
            }
            ret.reverse();
            ret
        }

        //leetcode 581
        pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
            if nums.len() <= 1 {
                return 0;
            }

            let mut sum = 0;
            let mut nums2 = nums.clone();
            nums2.sort();
            let (mut l, mut r) = (0, 0);
            for i in 0..nums.len() {
                if nums[i] != nums2[i] {
                    l = i;
                    break;
                }
            }

            for i in 0..nums.len() {
                if nums[i] != nums2[i] {
                    r = i;
                }
            }

            sum = r as i32 - l as i32;
            if sum <= 0 {
                0
            } else {
                sum as i32 + 1
            }
        }

        //leetcode 605
        pub fn can_place_flowers(f: Vec<i32>, n: i32) -> bool {
            let (mut sum, mut i) = (0, 0);
            while i < f.len() {
                if (i == 0 || f[i - 1] == 0) && f[i] == 0 && (i == f.len() - 1 || f[i + 1] == 0) {
                    i += 2;
                    sum += 1;
                } else {
                    i += 1;
                }
                if sum >= n {
                    return true;
                }
            }
            false
        }
    }
}

#[test]
fn test_can_place_flowers() {
    assert_eq!(
        code::Solution::can_place_flowers(vec![0, 0, 1, 0, 0], 1),
        true
    );
    assert_eq!(
        code::Solution::can_place_flowers(vec![0, 0, 1, 0, 1, 0, 0], 2),
        true
    );
    assert_eq!(
        code::Solution::can_place_flowers(vec![0, 0, 1, 0, 1], 1),
        true
    );
    assert_eq!(
        code::Solution::can_place_flowers(vec![1, 0, 0, 0, 1, 0, 1], 1),
        true
    );
    assert_eq!(
        code::Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1),
        true
    );
    assert_eq!(
        code::Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2),
        false
    );
    assert_eq!(
        code::Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 0, 1], 2),
        true
    );
    assert_eq!(
        code::Solution::can_place_flowers(vec![1, 0, 1, 0, 1, 0, 1], 0),
        true
    );
}

#[test]
fn test_find_unsorted_subarray() {
    assert_eq!(code::Solution::find_unsorted_subarray(vec![4, 2]), 2);
    assert_eq!(
        code::Solution::find_unsorted_subarray(vec![1, 2, 3, 3, 3]),
        0
    );
    assert_eq!(
        code::Solution::find_unsorted_subarray(vec![1, 1, 3, 2, 2, 2]),
        4
    );
    assert_eq!(
        code::Solution::find_unsorted_subarray(vec![1, 5, 5, 5, 7, 9, 3, 2, 2, 2]),
        9
    );
    assert_eq!(code::Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
    assert_eq!(code::Solution::find_unsorted_subarray(vec![2, 10, 9]), 2);
    assert_eq!(
        code::Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
        5
    );
    assert_eq!(
        code::Solution::find_unsorted_subarray(vec![2, 10, 9, 15]),
        2
    );
    assert_eq!(code::Solution::find_unsorted_subarray(vec![2]), 0);
}

#[test]
fn test_matrix_reshape() {
    assert_eq!(
        code::Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
        vec![vec![1, 2, 3, 4]]
    );
    assert_eq!(
        code::Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
        vec![vec![1, 2], vec![3, 4]]
    );
}

#[test]
fn test_array_pair_sum() {
    assert_eq!(code::Solution::array_pair_sum(vec![1, 3, 2, 4]), 4);
    assert_eq!(code::Solution::array_pair_sum(vec![1, 3, 2, 4, -7, -8]), -4);
}

#[test]
fn test_find_pairs() {
    assert_eq!(
        code::Solution::find_pairs(vec![3, 1, 4, 1, 5, 1, 3, 6], 2),
        3
    );
    assert_eq!(code::Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
    assert_eq!(code::Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
}

#[test]
fn test_find_max_consecutive_ones() {
    assert_eq!(
        code::Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
        3
    );
}

#[test]
fn test_find_disappeared_numbers() {
    assert_eq!(
        code::Solution::find_disappeared_numbers(vec![1, 3, 3, 3, 5]),
        vec![2, 4]
    );
    assert_eq!(
        code::Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
    assert_eq!(
        code::Solution::find_disappeared_numbers(vec![3, 2, 2, 3, 5, 1]),
        vec![4, 6]
    );
    assert_eq!(
        code::Solution::find_disappeared_numbers(vec![1, 1]),
        vec![2]
    );
    assert_eq!(code::Solution::find_disappeared_numbers(vec![1, 2]), vec![]);
}

#[test]
fn test_third_max() {
    assert_eq!(code::Solution::third_max(vec![1, 2, 2, 5, 3, 5]), 2);
    assert_eq!(code::Solution::third_max(vec![1, 1, 1, 1]), 1);
    assert_eq!(code::Solution::third_max(vec![]), 0);
    assert_eq!(code::Solution::third_max(vec![0]), 0);
    assert_eq!(code::Solution::third_max(vec![1, 2]), 2);
    assert_eq!(code::Solution::third_max(vec![3, 2, 1]), 1);
    assert_eq!(code::Solution::third_max(vec![3, 2, 1]), 1);
    assert_eq!(code::Solution::third_max(vec![2, 2, 3, 1]), 1);
}

#[test]
fn test_summary_range() {
    assert_eq!(
        code::Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
        vec!["0", "2->4", "6", "8->9"]
    );
    assert_eq!(
        code::Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec!["0->2", "4->5", "7"]
    );
    assert_eq!(code::Solution::summary_ranges(vec![1]), vec!["1"]);
    assert_eq!(
        code::Solution::summary_ranges(vec![-2147483648, -2147483647, 2147483647]),
        vec!["-2147483648->-2147483647", "2147483647"]
    );
}

#[test]
fn test_move_zeros() {
    let mut arr = vec![0, 1, 0, 3, 12];
    code::Solution::move_zeroes(&mut arr);
    assert_eq!(arr, vec![1, 3, 12, 0, 0]);

    let mut arr2 = vec![0, 0, 1];
    code::Solution::move_zeroes(&mut arr2);
    assert_eq!(arr2, vec![1, 0, 0]);
}

#[test]
fn test_missing_number() {
    assert_eq!(code::Solution::missing_number(vec![0, 1]), 2);
    assert_eq!(code::Solution::missing_number(vec![1, 2, 3, 5, 6]), 4);
    assert_eq!(code::Solution::missing_number(vec![3, 0, 1]), 2);
    assert_eq!(code::Solution::missing_number(vec![3]), 2);
    assert_eq!(code::Solution::missing_number(vec![3, 4]), 4);
    assert_eq!(
        code::Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
        8
    );
}

#[test]
fn test_contains_nearby_duplicate() {
    assert_eq!(
        code::Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
        false
    );
    assert_eq!(
        code::Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
        false
    );
    assert_eq!(
        code::Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
        true
    );
    assert_eq!(
        code::Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
        true
    );
}

#[test]
fn test_contains_duplicate() {
    assert_eq!(code::Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(code::Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(
        code::Solution::contains_duplicate(vec![1, 2, 3, 4, 3]),
        true
    );
}

#[test]
fn test_rotate() {
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
    code::Solution::rotate(&mut arr, 1);
    assert_eq!(arr, vec![7, 1, 2, 3, 4, 5, 6]);

    let mut arr2 = vec![1, 2, 3, 4, 5, 6, 7];
    code::Solution::rotate(&mut arr2, 3);
    assert_eq!(arr2, vec![5, 6, 7, 1, 2, 3, 4]);
}

#[test]
fn test_majority_element() {
    assert_eq!(code::Solution::majority_element(vec![3, 2, 3]), 3);
    assert_eq!(
        code::Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]),
        2
    );
}

#[test]
fn test_two_sum() {
    assert_eq!(
        vec![1, 2],
        code::Solution::two_sum(vec![2, 7, 11, 15, 19], 9)
    );
    assert_eq!(vec![1, 2], code::Solution::two_sum(vec![0, 0, 3, 4], 0));
}

#[test]
fn test_get_row() {
    assert_eq!(vec![1, 4, 6, 4, 1], code::Solution::get_row(7));
}

#[test]
fn test_generate() {
    assert_eq!(
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ],
        code::Solution::generate(5)
    );
}

#[test]
fn test_merge() {
    let mut foo = vec![1, 2, 3, 0, 0, 0];
    let mut bar = vec![2, 5, 6];
    code::Solution::merge(&mut foo, 3, &mut bar, 3);
    assert_eq!(vec![1, 2, 2, 3, 5, 6], foo);

    let mut foo = vec![1, 2, 3, 0, 0, 0, 0];
    let mut bar = vec![2, 5, 6];
    code::Solution::merge(&mut foo, 2, &mut bar, 2);
    assert_eq!(vec![1, 2, 2, 5, 0, 0, 0], foo);
}

#[test]
fn test_mu_sqrt() {
    assert_eq!(3, code::Solution::my_sqrt(9));
    assert_eq!(2, code::Solution::my_sqrt(8));
}

#[test]
fn test_add_binary() {
    //"10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101"
    //"110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011"
    //    assert_eq!(
    //        code::Solution::add_binary(String::from("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101"), String::from("110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011")),
    //        "100"
    //    );
    assert_eq!(3.0, 9.0_f32.sqrt());
}

#[test]
fn test_plus_one() {
    assert_eq!(code::Solution::plus_one(vec![1, 2, 8]), vec![1, 2, 9]);
    assert_eq!(code::Solution::plus_one(vec![1, 2, 3, 9]), vec![1, 2, 4, 0]);
    assert_eq!(code::Solution::plus_one(vec![9]), vec![1, 0]);
    assert_eq!(code::Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(code::Solution::plus_one(vec![]), vec![1]);
    assert_eq!(code::Solution::plus_one(vec![0]), vec![1]);
    assert_eq!(code::Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(
        code::Solution::plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1]
    );
}

#[test]
fn test_length_of_last_word() {
    assert_eq!(
        code::Solution::length_of_last_word(String::from("Aaa Bbb CCCc")),
        4
    );
    assert_eq!(
        code::Solution::length_of_last_word(String::from("Hello World")),
        5
    );
    assert_eq!(code::Solution::length_of_last_word(String::from("a")), 1);
    assert_eq!(code::Solution::length_of_last_word(String::from("")), 0);
    assert_eq!(code::Solution::length_of_last_word(String::from("a ")), 1);
}

#[test]
fn test_count_and_say() {}

#[test]
fn test_search_insert() {
    assert_eq!(code::Solution::search_insert(vec![1, 3, 5, 6], 3), 1);
    assert_eq!(code::Solution::search_insert(vec![1, 3, 5, 6], 6), 3);
    assert_eq!(code::Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(code::Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(code::Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(code::Solution::search_insert(vec![1], 7), 1);
    assert_eq!(code::Solution::search_insert(vec![0], 7), 1);
    assert_eq!(code::Solution::search_insert(vec![], 7), 0);
}

#[test]
fn test_str_str() {
    assert_eq!(
        code::Solution::str_str(String::from("hello"), String::from("ll")),
        2,
    );
    assert_eq!(
        code::Solution::str_str(String::from(""), String::from("")),
        0,
    );
    assert_eq!(
        code::Solution::str_str(String::from("charlie"), String::from("e")),
        6,
    );
    assert_eq!(
        code::Solution::str_str(String::from(""), String::from("e")),
        -1,
    );
    assert_eq!(
        code::Solution::str_str(String::from("aaa"), String::from("")),
        0,
    );
}

#[test]
fn test_remoce_element() {
    assert_eq!(code::Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
    assert_eq!(code::Solution::remove_element(&mut vec![0], 3), 1);
}

#[test]
fn test_remove_duplicates() {
    assert_eq!(
        code::Solution::remove_duplicates2(&mut vec![0, 1, 1, 1, 2, 2, 3, 3, 4, 4]),
        5
    );
    assert_eq!(code::Solution::remove_duplicates2(&mut vec![0]), 1);
    assert_eq!(code::Solution::remove_duplicates2(&mut vec![1, 1, 2]), 2);
    assert_eq!(code::Solution::remove_duplicates2(&mut vec![0, 0]), 1);
    assert_eq!(code::Solution::remove_duplicates2(&mut vec![]), 0);

    let mut arr = vec![1, 1, 2];
    code::Solution::remove_duplicates2(&mut arr);
    println!("{:?}", arr)
}

#[test]
fn test_generate_parenthesis() {
    assert_eq!(
        code::Solution::generate_parenthesis(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    )
}

#[test]
fn test_is_valid() {
    assert_eq!(code::Solution::is_valid("{([])}".to_string()), true);
    assert_eq!(code::Solution::is_valid("()".to_string()), true);
    assert_eq!(code::Solution::is_valid("[]".to_string()), true);
    assert_eq!(code::Solution::is_valid("[)".to_string()), false);
    assert_eq!(code::Solution::is_valid("(){}[]".to_string()), true);
    assert_eq!(code::Solution::is_valid("([)]".to_string()), false);
    assert_eq!(code::Solution::is_valid("".to_string()), true);
    assert_eq!(code::Solution::is_valid("(".to_string()), false);
    assert_eq!(code::Solution::is_valid("(((()".to_string()), false);
    assert_eq!(code::Solution::is_valid("((".to_string()), false);
    assert_eq!(code::Solution::is_valid("))".to_string()), false);
    assert_eq!(code::Solution::is_valid(")".to_string()), false);
    assert_eq!(code::Solution::is_valid("   ".to_string()), false);
    assert_eq!(code::Solution::is_valid(" ".to_string()), false);
    assert_eq!(code::Solution::is_valid("   (  ) ".to_string()), false);
}

#[test]
fn main_test() {
    let mut arr: Vec<&str> = vec!["flower", "flow", "flight"];
    let mut target = 0;
    assert_eq!('c' as u8, "charlie".as_bytes()[0]);
    assert_eq!(
        arr.iter().any(|&x| {
            print!("{:?}", x);
            (*x).as_bytes()[target] == 'c' as u8
        }),
        false
    )
}

#[test]
fn test_rob() {
    //    assert_eq!(code::Solution::rob(vec![2,7,9,3,1]),12);
    //    assert_eq!(code::Solution::rob(vec![0]),0);
    //    assert_eq!(code::Solution::rob(vec![2,1,1,2]),4);
    //    assert_eq!(code::Solution::rob(vec![6,6,4,8,4,3,3,10]),27);
    assert_eq!(code::Solution::rob(vec![2, 4, 8, 9, 9, 3]), 19);
}

#[test]
fn test_re_words() {
    let mut str1 = "this is a   pen  ";
    assert_eq!(code::Solution::re_words(str1), "pen a is this");
}

#[test]
fn test_max_product() {
    assert_eq!(code::Solution::max_product(vec![2, 3, 0, 4]), 6);
    assert_eq!(code::Solution::max_product(vec![-2, 0, -1]), 0);
    assert_eq!(code::Solution::max_product(vec![-2, 0, -1, -3, -4]), 12);
    assert_eq!(code::Solution::max_product(vec![-2]), -2);
    assert_eq!(code::Solution::max_product(vec![2]), 2);
    assert_eq!(code::Solution::max_product(vec![0]), 0);
    assert_eq!(code::Solution::max_product(vec![-1, -2, -3, 9, -8, -9]), 72);
}

#[test]
fn test_max_sub_array() {
    assert_eq!(
        code::Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(code::Solution::max_sub_array(vec![-1]), -1);
    assert_eq!(code::Solution::max_sub_array(vec![-2, -1]), -1);
}

#[test]
fn test_insert() {
    assert_eq!(
        code::Solution::insert(vec![(1, 2), (3, 4), (2, 3), (7, 8), (5, 6)]),
        vec![(1, 4), (7, 8), (5, 6)]
    );
}

#[test]
fn test_longest_common_prefix() {
    assert_eq!(
        code::Solution::longest_common_prefix(vec![
            String::from("charlie"),
            String::from("charxxx"),
            String::from("chakkk")
        ]),
        "cha"
    );
    assert_eq!(
        code::Solution::longest_common_prefix(vec![
            String::from("aaa1"),
            String::from("aaaa2"),
            String::from("aa3")
        ]),
        "aa"
    );
    assert_eq!(
        code::Solution::longest_common_prefix(vec![
            String::from(""),
            String::from(""),
            String::from("")
        ]),
        ""
    );
    assert_eq!(
        code::Solution::longest_common_prefix(vec![
            String::from(""),
            String::from("aaaaaa"),
            String::from("")
        ]),
        ""
    );
    assert_eq!(
        code::Solution::longest_common_prefix(vec![
            String::from("aaaaaaaaa"),
            String::from(""),
            String::from("")
        ]),
        ""
    );
    assert_eq!(
        code::Solution::longest_common_prefix(vec![
            String::from("aaa"),
            String::from("aa"),
            String::from("a")
        ]),
        "a"
    );
    assert_eq!(
        code::Solution::longest_common_prefix(vec![String::from("aaa")]),
        "aaa"
    );
    assert_eq!(code::Solution::longest_common_prefix(vec![]), "");
}

#[test]
fn test_integer_to_roman() {
    assert_eq!("MCMXCIV", code::Solution::int_to_roman(1994));
}

#[test]
fn test_roman_to_integer() {
    assert_eq!(1994, code::Solution::roman_to_int("MCMXCIV".to_string()));
    assert_eq!(3, code::Solution::roman_to_int("III".to_string()));
    assert_eq!(0, code::Solution::roman_to_int("".to_string()));
    assert_eq!(4, code::Solution::roman_to_int("IV".to_string()));
    assert_eq!(9, code::Solution::roman_to_int("IX".to_string()));
    assert_eq!(58, code::Solution::roman_to_int("LVIII".to_string()));
    assert_eq!(1570, code::Solution::roman_to_int("MDLXX".to_string()));
}
//EOF
