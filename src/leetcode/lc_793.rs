// ...existing code...
pub fn preimage_size_fzf(k: i32) -> i32 {
    fn trailing_zeroes(mut n: i64) -> i64 {
        let mut count = 0;
        while n > 0 {
            n /= 5;
            count += n;
        }
        count
    }

    fn lower_bound(k: i64) -> i64 {
        let (mut low, mut high) = (0i64, 5_000_000_010);
        while low < high {
            let mid = (low + high) / 2;
            if trailing_zeroes(mid) < k {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }

    let start = lower_bound(k as i64);
    let end = lower_bound(k as i64 + 1);
    (end - start) as i32
}
// ...existing code...
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preimage_size_fzf() {
        assert_eq!(preimage_size_fzf(0), 1);
        assert_eq!(preimage_size_fzf(5), 0);
        assert_eq!(preimage_size_fzf(100), 5);
    }
}
// ...existing code...
