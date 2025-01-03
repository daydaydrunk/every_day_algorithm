struct Solution {}

impl Solution {
    pub fn similar_rgb(color: String) -> String {
        fn closest_shorthand_component(component: &str) -> String {
            let value = i32::from_str_radix(component, 16).unwrap();
            let shorthand_value = (value / 17 + if value % 17 > 8 { 1 } else { 0 }) * 17;
            format!("{:02x}", shorthand_value)
        }

        let r = &color[1..3];
        let g = &color[3..5];
        let b = &color[5..7];

        format!(
            "#{}{}{}",
            closest_shorthand_component(r),
            closest_shorthand_component(g),
            closest_shorthand_component(b)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similar_rgb() {
        // Test case 1: color = "#09f166"
        assert_eq!(Solution::similar_rgb("#09f166".to_string()), "#11ee66");

        // Test case 2: color = "#4e3fe1"
        assert_eq!(Solution::similar_rgb("#4e3fe1".to_string()), "#5544dd");

        // Test case 3: color = "#abcdef"
        assert_eq!(Solution::similar_rgb("#abcdef".to_string()), "#aabbcc");
    }
}
