use std::u32;
use std::char;
use std::cmp::Ordering;

struct StringMatch<'a> {
    pub major_axis_distance: u32,
    pub minor_axis_distance: u32,
    pub index: usize,
    pub value: &'a str
}

impl<'a> PartialEq for StringMatch<'a> {
    fn eq(&self, other: &StringMatch) -> bool {
        self.major_axis_distance == other.major_axis_distance
            && self.major_axis_distance == other.minor_axis_distance
    }
}

impl<'a> Eq for StringMatch<'a> {}

impl<'a> PartialOrd for StringMatch<'a> {
    fn partial_cmp(&self, other: &StringMatch) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for StringMatch<'a> {
    fn cmp(&self, other: &StringMatch) -> Ordering {
        if self.major_axis_distance > other.major_axis_distance {
            return Ordering::Greater;
        } else if self.major_axis_distance < other.major_axis_distance {
            return Ordering::Less;
        } else {
            if self.minor_axis_distance > other.minor_axis_distance {
                return Ordering::Greater;
            } else if self.minor_axis_distance < other.minor_axis_distance {
                return Ordering::Less;
            } else {
                return Ordering::Equal;
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct MatchIndex<'a>(usize, &'a str);

pub fn get_levenshtein_distance(lhs: &str, rhs: &str) -> u32 {
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

fn get_cap_distance(lhs: &str, rhs: &str) -> u32 {
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
        .flat_map(|w| split_case(w))
        .map(|w| w.chars().nth(0).unwrap())
        .collect::<String>()
        .to_uppercase()
}

fn split_case(word: &str) -> Vec<String> {
    let word_chars: Vec<char> = word.chars().collect();
    let mut words: Vec<String> = Vec::new();
    
    if word_chars.len() < 1 {
        return words;
    }

    let mut last_char = &char::MAX;
    let mut word_start_index = 0;


    for (i, c) in word_chars.iter().enumerate() {
        if c.is_uppercase() && !last_char.is_whitespace() && i > word_start_index {
            words.push(word_chars.iter().skip(word_start_index).take(i - word_start_index).collect::<String>());
            word_start_index = i;
        }
        last_char = c;
    }

    words.push(word_chars.iter().skip(word_start_index).take(word_chars.len() - word_start_index).collect::<String>());

    words
}

pub fn fuzzymatch<'a>(search_keys: &Vec<&'a str>, term: &str, threshold: f32) -> Vec<MatchIndex<'a>> {
    let mut matches = Vec::new();

    //if an exact match, there is only 1
    for (i, key) in search_keys.iter().enumerate() {
        if *key == term {
            return vec![MatchIndex(i, key)];
        }
    }

    //next match by exact match with casing distance
    for (i, key) in search_keys.iter().enumerate() {
        if *key.to_lowercase() == term.to_lowercase() {
            matches.push(StringMatch{
                major_axis_distance: 1,
                minor_axis_distance: get_cap_distance(key, term),
                index: i,
                value: key
            });
        }
    }

    //next match by initials
    for (i, key) in search_keys.iter().enumerate() {
        let initials = to_initials(key);
        if initials.to_lowercase() == term.to_lowercase() {
            matches.push(StringMatch{
                major_axis_distance: 2,
                minor_axis_distance: get_cap_distance(key, term),
                index: i,
                value: key
            });
        }
    }

    //next match by contains match weighted by how much it matched
    for (i, key) in search_keys.iter().enumerate() {
        if key.contains(term) {
            let len = key.len();
            let distance = (len as i32 - term.len() as i32).abs();
            if distance as f32 <= len as f32 - (len as f32 * threshold) {
                matches.push(StringMatch{
                    major_axis_distance: 3,
                    minor_axis_distance: distance as u32,
                    index: i,
                    value: key
                });
            }
        }
    }
    
    //next match by levenshtien distance
    for (i, key) in search_keys.iter().enumerate() {
        let distance = get_levenshtein_distance(key, term);
        let len = key.len();
        if distance as f32 <= len as f32 - (len as f32 * threshold) {
            matches.push(StringMatch{
                major_axis_distance: 4,
                minor_axis_distance: distance,
                index: i,
                value: key
            });
        }
    }

    matches.sort();

    let mut match_indicies = matches
        .iter()
        .map(|m| MatchIndex(m.index, m.value))
        .collect::<Vec<MatchIndex>>();

    match_indicies.dedup();

    match_indicies
}


#[cfg(test)]
mod fuzzymatch_tests {
    use super::*;

    #[test]
    fn split_case_should_split_on_camel_case(){
        assert_eq!(vec!["a", "Happy", "Day"], split_case("aHappyDay"));
    }

    #[test]
    fn split_case_should_split_on_title_case(){
        assert_eq!(vec!["A", "Happy", "Day"], split_case("AHappyDay"));
    }

    #[test]
    fn levenshtein_should_get_correct_distance() {
        assert_eq!(get_levenshtein_distance("x", "x"), 0);
        assert_eq!(get_levenshtein_distance("x", "y"), 1);
        assert_eq!(get_levenshtein_distance("", "x"), 1);
        assert_eq!(get_levenshtein_distance("y", ""), 1);
        assert_eq!(get_levenshtein_distance("kitten", "mutton"), 3);
        assert_eq!(get_levenshtein_distance("abc", "abbc"), 1);
        assert_eq!(get_levenshtein_distance("book", "back"), 2);
    }

    #[test]
    fn fuzzymatch_should_find_exact_match(){
        let words = vec!["foo", "bar", "abc"];

        assert_eq!(vec![MatchIndex(0, "foo")], fuzzymatch(&words, "foo", 0.7));
        assert_eq!(vec![MatchIndex(1, "bar")], fuzzymatch(&words, "bar", 0.7));
    }

    #[test]
    fn should_match_a_character_insertion(){
        let words = vec!["foo", "bar", "zzz"];

        assert_eq!(vec![MatchIndex(0, "foo")], fuzzymatch(&words, "foos", 0.5));
        assert_eq!(vec![MatchIndex(1, "bar")], fuzzymatch(&words, "bars", 0.5));
        assert_eq!(vec![MatchIndex(0, "foo")], fuzzymatch(&words, "afoo", 0.5));
        assert_eq!(vec![MatchIndex(1, "bar")], fuzzymatch(&words, "abar", 0.5));
        assert_eq!(vec![MatchIndex(0, "foo")], fuzzymatch(&words, "fo.o", 0.5));
        assert_eq!(vec![MatchIndex(1, "bar")], fuzzymatch(&words, "b.ar", 0.5));
    }

    #[test]
    fn should_match_a_character_deletion() {
        let words = vec!["qux", "bar", "zzz"];

        assert_eq!(vec![MatchIndex(0, "qux")], fuzzymatch(&words, "qu", 0.5));
        assert_eq!(vec![MatchIndex(1, "bar")], fuzzymatch(&words, "ba", 0.5));
        assert_eq!(vec![MatchIndex(0, "qux")], fuzzymatch(&words, "ux", 0.5));
        assert_eq!(vec![MatchIndex(1, "bar")], fuzzymatch(&words, "ar", 0.5));
        assert_eq!(vec![MatchIndex(0, "qux")], fuzzymatch(&words, "qx", 0.5));
        assert_eq!(vec![MatchIndex(1, "bar")], fuzzymatch(&words, "br", 0.5));
    }

    #[test]
    fn should_match_a_character_swap() {
        let words = vec!["qux", "bar", "zzz"];

        assert_eq!(vec![MatchIndex(0, "qux")], fuzzymatch(&words, "quk", 0.5));
        assert_eq!(vec![MatchIndex(1, "bar")], fuzzymatch(&words, "bam", 0.5));
        assert_eq!(vec![MatchIndex(0, "qux")], fuzzymatch(&words, "lux", 0.5));
        assert_eq!(vec![MatchIndex(1, "bar")], fuzzymatch(&words, "car", 0.5));
        assert_eq!(vec![MatchIndex(0, "qux")], fuzzymatch(&words, "qix", 0.5));
        assert_eq!(vec![MatchIndex(1, "bar")], fuzzymatch(&words, "bor", 0.5));
    }

    #[test]
    fn edit_distance_should_be_prioritized() {
        let words = vec!["candyjake", "candyjane", "abc"];

        assert_eq!(vec![MatchIndex(1, "candyjane"), MatchIndex(0, "candyjake")], fuzzymatch(&words, "candycane", 0.7));
    }

    #[test]
    fn should_not_match_if_under_threshold() {
        let words = vec!["applehorse", "pearcat", "grapechicken", "abc"];

        assert_eq!(Vec::<MatchIndex>::new(), fuzzymatch(&words, "applecat", 0.8));
        assert_eq!(Vec::<MatchIndex>::new(), fuzzymatch(&words, "pearchicken", 0.8));
        assert_eq!(Vec::<MatchIndex>::new(), fuzzymatch(&words, "grapehorse", 0.8));
    }

    #[test]
    fn should_match_string_that_contains() {
        let words = vec!["applehorse", "pearcat", "grapechicken", "abc"];

        assert_eq!(vec![MatchIndex(0, "applehorse")], fuzzymatch(&words, "appleh", 0.5));
        assert_eq!(vec![MatchIndex(1, "pearcat")], fuzzymatch(&words, "pearc", 0.5));
        assert_eq!(vec![MatchIndex(2, "grapechicken")], fuzzymatch(&words, "grapec", 0.5));
    }

    #[test]
    fn should_match_initals_with_caps() {
        let words = vec!["Fuzzy Match", "Jungle Adventure", "Pacific Cruiseship", "Desert Airway"];

        assert_eq!(vec![MatchIndex(0, "Fuzzy Match")], fuzzymatch(&words, "FM", 0.7));
        assert_eq!(vec![MatchIndex(1, "Jungle Adventure")], fuzzymatch(&words, "JA", 0.7));
        assert_eq!(vec![MatchIndex(2, "Pacific Cruiseship")], fuzzymatch(&words, "PC", 0.7));
    }

    #[test]
    fn should_match_case_invariant_initals_with_caps() {
        let words = vec!["fuzzy match", "jungle adventure", "pacific cruiseship", "desert airway"];

        assert_eq!(vec![MatchIndex(0, "fuzzy match")], fuzzymatch(&words, "FM", 0.7));
        assert_eq!(vec![MatchIndex(1, "jungle adventure")], fuzzymatch(&words, "JA", 0.7));
        assert_eq!(vec![MatchIndex(2, "pacific cruiseship")], fuzzymatch(&words, "PC", 0.7));
    }

    #[test]
    fn should_match_title_case_initals_with_caps() {
        let words = vec!["FuzzyMatch", "JungleAdventure", "PacificCruiseship", "DesertAirway"];

        assert_eq!(vec![MatchIndex(0, "FuzzyMatch")], fuzzymatch(&words, "FM", 0.7));
        assert_eq!(vec![MatchIndex(1, "JungleAdventure")], fuzzymatch(&words, "JA", 0.7));
        assert_eq!(vec![MatchIndex(3, "DesertAirway")], fuzzymatch(&words, "DA", 0.7));
    }

    #[test]
    fn should_match_initals_with_lowercase() {
        let words = vec!["Fuzzy Match", "Jungle Adventure", "Pacific Cruiseship", "Desert Airway"];

        assert_eq!(vec![MatchIndex(0, "Fuzzy Match")], fuzzymatch(&words, "fm", 0.7));
        assert_eq!(vec![MatchIndex(1, "Jungle Adventure")], fuzzymatch(&words, "ja", 0.7));
        assert_eq!(vec![MatchIndex(2, "Pacific Cruiseship")], fuzzymatch(&words, "pc", 0.7));
    }

    /*
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
        assert_eq!(Vec::<MatchIndex>::new(), fuzzymatch(&words, "melon", 0.7));
    }
}