fn levenshtein(lhs: &str, rhs: &str) -> usize {
    let lhs_len = lhs.chars().count();
    let rhs_len = rhs.chars().count();
    let mut grid = vec![vec![0 as usize; lhs_len + 1]; rhs_len + 1];

    for i in 0..=lhs_len {
        grid[i][0] = i;
    }

    for j in 0..=rhs_len {
        grid[0][j] = j;
    }

    for j in 1..=rhs_len {
        for i in 1..=lhs_len {
            let matched = if lhs.chars().nth(i - 1).unwrap() == rhs.chars().nth(j - 1).unwrap() { 0 } else { 1 };
            vec![
                grid[j][i - 1] + 1,
                grid[j - 1][i] + 1,
                grid[j - 1][i - 1] + matched
            ]
            .iter()
            .min_by(|x, y| x.cmp(y));
        }
    }

    grid[rhs_len][lhs_len]
}

fn fuzzymatch<'a>(search_keys: Vec<&'a str>, term: &str) -> Option<(usize, &'a str)> {
    for (i, key) in search_keys.iter().enumerate() {
        if key == &term {
            return Some((i, key));
        }
    }

    None
}


#[cfg(test)]
mod fuzzymatch_tests {
    use super::*;

    #[test]
    fn levenshtein_should_get_correct_distance() {
        assert_eq!(levenshtein("x", "x"), 0);
        assert_eq!(levenshtein("x", "y"), 1);
        //assert_eq!(levenshtein("", "x"), 1);
        //assert_eq!(levenshtein("y", ""), 1);
        //assert_eq!(levenshtein("kitten", "mutton"), 3);
    }

    #[test]
    fn fuzzymatch_should_find_exact_match(){
        assert_eq!(Some((1, "pear")), fuzzymatch(vec!["apple", "pear", "banana", "orange"], "pear"));
    }

        #[test]
    fn fuzzymatch_should_fail_if_no_match(){
        assert_eq!(None, fuzzymatch(vec!["apple", "pear", "banana", "orange"], "melon"));
    }
}