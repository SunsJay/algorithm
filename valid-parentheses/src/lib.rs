impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() & 1 == 1 { return false; }
        let mut v = vec![];
        for c in s.into_bytes() {
            match c {
                b'(' | b'[' | b'{' => v.push(c + (1 << (c & 1))),
                _ => if Some(c) != v.pop() { return false }
            }
        }
        v.is_empty()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
