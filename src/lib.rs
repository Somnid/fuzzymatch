use std::u32;

pub fn levenshtein(lhs: &str, rhs: &str) -> u32 {
    let lhs_len = lhs.chars().count();
    let rhs_len = rhs.chars().count();
    let mut grid = vec![vec![0; lhs_len + 1]; rhs_len + 1];

    for i in 0..=lhs_len {
        grid[0][i] = i as u32;
    }

    for j in 0..=rhs_len {
        grid[j][0] = j as u32;
    }

    for j in 1..=rhs_len {
        for i in 1..=lhs_len {
            let matched = if lhs.chars().nth(i - 1).unwrap() == rhs.chars().nth(j - 1).unwrap() { 0 } else { 1 };

            grid[j][i] = *vec![
                grid[j][i - 1] + 1,
                grid[j - 1][i] + 1,
                grid[j - 1][i - 1] + matched
            ]
            .iter()
            .min_by_key(|&x| x).unwrap();
        }
    }

    grid[rhs_len][lhs_len]
}

fn cap_distance(lhs: &str, rhs: &str) -> u32 {
    if rhs.to_lowercase() != lhs.to_lowercase() { //must be the same case invariant or no real match
        return u32::MAX;
    }
    let rhs_chars = rhs.chars();
    let lhs_chars = lhs.chars();
    let mut distance = 0;

    for (left_char, right_char) in lhs_chars.zip(rhs_chars) {
        if left_char != right_char { //since strings match case-invariant, this can only mean case mismatch
            distance += 1;
        }
    }

    distance
}

pub fn to_initials(word: &str) -> String {
    word
        .split_whitespace()
        .filter(|w| w.len() > 0)
        .map(|w| w.chars().nth(0).unwrap())
        .collect::<String>()
        .to_uppercase()
}

pub fn fuzzymatch<'a>(search_keys: &Vec<&'a str>, term: &str, threshold: u32) -> Option<(usize, &'a str)> {
    let mut best_by_initials: Option<(u32, usize, &str)> = None; //distance, index, term
    let mut best_by_distance: Option<(u32, usize, &str)> = None; //distance, index, term

    for (i, key) in search_keys.iter().enumerate() {
        if *key == term {
            return Some((i, key));
        }
        let intials = to_initials(key);
        if intials.eq(term) {   
            let initial_distance = cap_distance(key, term);
            match best_by_initials {
                Some(best_initials) => {
                    if initial_distance < best_initials.0 {
                        best_by_initials = Some((initial_distance, i, key));
                    }
                }
                None => {
                    best_by_initials = Some((initial_distance, i, key));
                }
            } 
        }

        let distance = levenshtein(key, term);

        match best_by_distance {
            Some(best_distance) => {
                if distance < best_distance.0 {
                    best_by_distance = Some((distance, i, key));
                }
            }
            None => {
                best_by_distance = Some((distance, i, key));
            }
        } 
    }

    if let Some(best_initial) = best_by_initials {
        return Some((best_initial.1, best_initial.2));
    }

    if let Some(best_distance) = best_by_distance {
        if best_distance.0 <= threshold {
            return Some((best_distance.1, best_distance.2));
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
        assert_eq!(levenshtein("", "x"), 1);
        assert_eq!(levenshtein("y", ""), 1);
        assert_eq!(levenshtein("kitten", "mutton"), 3);
        assert_eq!(levenshtein("abc", "abbc"), 1);
        assert_eq!(levenshtein("book", "back"), 2);
    }

    #[test]
    fn fuzzymatch_should_find_exact_match(){
        let words = vec!["foo", "bar", "abc"];
        assert_eq!(Some((0, "foo")), fuzzymatch(&words, "foo", 3));
        assert_eq!(Some((1, "bar")), fuzzymatch(&words, "bar", 3));
    }

    #[test]
    fn should_match_a_character_insertion(){
        let words = vec!["foo", "bar", "abc"];

        assert_eq!(Some((0, "foo")), fuzzymatch(&words, "foos", 3));
        assert_eq!(Some((1, "bar")), fuzzymatch(&words, "bars", 3));
        assert_eq!(Some((0, "foo")), fuzzymatch(&words, "afoo", 3));
        assert_eq!(Some((1, "bar")), fuzzymatch(&words, "abar", 3));
        assert_eq!(Some((0, "foo")), fuzzymatch(&words, "fo.o", 3));
        assert_eq!(Some((1, "bar")), fuzzymatch(&words, "b.ar", 3));
    }

    #[test]
    fn should_match_a_character_deletion() {
        let words = vec!["qux", "bar", "abc"];

        assert_eq!(Some((0, "qux")), fuzzymatch(&words, "qu", 3));
        assert_eq!(Some((1, "bar")), fuzzymatch(&words, "ba", 3));
        assert_eq!(Some((0, "qux")), fuzzymatch(&words, "ux", 3));
        assert_eq!(Some((1, "bar")), fuzzymatch(&words, "ar", 3));
        assert_eq!(Some((0, "qux")), fuzzymatch(&words, "qx", 3));
        assert_eq!(Some((1, "bar")), fuzzymatch(&words, "br", 3));
    }

    #[test]
    fn should_match_a_character_swap() {
        let words = vec!["qux", "bar", "abc"];

        assert_eq!(Some((0, "qux")), fuzzymatch(&words, "quk", 3));
        assert_eq!(Some((1, "bar")), fuzzymatch(&words, "bam", 3));
        assert_eq!(Some((0, "qux")), fuzzymatch(&words, "lux", 3));
        assert_eq!(Some((1, "bar")), fuzzymatch(&words, "car", 3));
        assert_eq!(Some((0, "qux")), fuzzymatch(&words, "qix", 3));
        assert_eq!(Some((1, "bar")), fuzzymatch(&words, "bur", 3));
    }

    #[test]
    fn edit_distance_should_be_prioritized() {
        let words = vec!["candyjake", "candyjane", "abc"];

        assert_eq!(Some((1, "candyjane")), fuzzymatch(&words, "candycane", 3));
    }

    #[test]
    fn should_not_match_more_than_n_edits() {
        let words = vec!["applehorse", "pearcat", "grapechicken", "abc"];

        assert_eq!(None, fuzzymatch(&words, "applecat", 3));
        assert_eq!(None, fuzzymatch(&words, "pearchicken", 3));
        assert_eq!(None, fuzzymatch(&words, "grapehorse", 3));
    }

    /*
        it("should match prefix", () => {
        const epics = ["applehorse", "pearcat", "grapechicken", "abc"];

        expect(fuzzyMatch(epics, "appleh")).toBe(0);
        expect(fuzzyMatch(epics, "pearc")).toBe(1);
        expect(fuzzyMatch(epics, "grapec")).toBe(2);
    });*/

    #[test]
    fn should_match_initals_with_caps() {
        let words = vec!["Fuzzy Match", "Jungle Adventure", "Pacific Cruiseship", "Desert Airway"];

        assert_eq!(fuzzymatch(&words, "FM", 3), Some((0, "Fuzzy Match")));
        assert_eq!(fuzzymatch(&words, "JA", 3), Some((1, "Jungle Adventure")));
        assert_eq!(fuzzymatch(&words, "PC", 3), Some((2, "Pacific Cruiseship")));
    }
/*
    t("should match case-invariant initals with caps", () => {
        const epics = ["fuzzy match", "jira epic", "jarvis tool", "abc"];

        expect(fuzzyMatch(epics, "FM")).toBeNull;
        expect(fuzzyMatch(epics, "JE")).toBeNull;
        expect(fuzzyMatch(epics, "JT")).toBeNull;
    });
        it("should match title-case initals with caps", () => {
        const epics = ["FuzzyMatch", "JiraEpic", "JarvisTool", "abc"];

        expect(fuzzyMatch(epics, "FM")).toBeNull;
        expect(fuzzyMatch(epics, "JE")).toBeNull;
        expect(fuzzyMatch(epics, "JT")).toBeNull;
    });
    it("should not match initals with lowercase", () => {
        const epics = ["Fuzzy Match", "Jira Epic", "Jarvis Tool", "abc"];

        expect(fuzzyMatch(epics, "fm")).toBeNull;
        expect(fuzzyMatch(epics, "je")).toBeNull;
        expect(fuzzyMatch(epics, "jt")).toBeNull;
    });
    it("exact match should prioritize over case insensitive", () => {
        const epics = ["blue", "BLUE", "abc"];

        expect(fuzzyMatch(epics, "BLUE")).toBe(1);
    });
    it("case insensitive match should prioritize over initials", () => {
        const epics = ["blue", "Big Lucky Uganda", "BLu", "abc"];

        expect(fuzzyMatch(epics, "BLU")).toBe(2);
    });
    it("intial match should prioritize over prefix", () => {
        const epics = ["BARK", "Big Orange Rat", "abc"];

        expect(fuzzyMatch(epics, "BAR")).toBe(1);
    });
    it("prefix match should prioritize over edit distance match", () => {
        const epics = ["BARKBONE", "BARB", "abc"];

        expect(fuzzyMatch(epics, "bark")).toBe(1);
    });
    */

    #[test]
    fn fuzzymatch_should_fail_if_no_match(){
        let words = vec!["apple", "pear", "banana", "orange"];
        assert_eq!(None, fuzzymatch(&words, "melon", 3));
    }
}