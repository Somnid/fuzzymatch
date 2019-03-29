extern crate wasm_bindgen;
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use std::hash::Hasher;
use std::hash::Hash;
use std::u32;
use std::char;
use std::cmp::Ordering;
use std::collections::HashSet;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug,Clone)]
struct StringMatch {
    pub major_axis_distance: u32,
    pub minor_axis_distance: u32,
    pub index: usize,
    pub value: String
}

impl PartialEq for StringMatch {
    fn eq(&self, other: &StringMatch) -> bool {
        self.index == other.index
            && self.value == other.value
    }
}

impl Eq for StringMatch {}

impl Hash for StringMatch {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.value.hash(state);
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct MatchIndex(usize, String);

impl PartialEq for MatchIndex {
    fn eq(&self, other: &MatchIndex) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

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
            let matched = if lhs.to_lowercase().chars().nth(i - 1).unwrap() == rhs.to_lowercase().chars().nth(j - 1).unwrap() { 0 } else { 1 };

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

#[wasm_bindgen]
pub fn fuzzymatchjs(search_keys: &JsValue, term: String, threshold: f32) -> JsValue {
    JsValue::from_serde(&fuzzymatch(search_keys.into_serde().unwrap(), term, threshold)).unwrap()
}

fn fuzzymatch(search_keys: Vec<String>, term: String, threshold: f32) -> Vec<MatchIndex> {
    let mut matches = HashSet::new();

    //if an exact match, there is only 1
    for (i, key) in search_keys.iter().enumerate() {
        if *key == term {
            return vec![MatchIndex(i, key.clone())];
        }
    }

    //next match by exact match with casing distance
    for (i, key) in search_keys.iter().enumerate() {
        if key.to_lowercase() == term.to_lowercase() {
            let found = StringMatch{
                major_axis_distance: 1,
                minor_axis_distance: get_cap_distance(key, &term),
                index: i,
                value: key.clone()
            };
            if !matches.contains(&found) {
                matches.insert(found);
            }
        }
    }

    //next match by initials
    for (i, key) in search_keys.iter().enumerate() {
        let initials = to_initials(key);
        if initials.to_lowercase() == term.to_lowercase() {
            let found = StringMatch{
                major_axis_distance: 2,
                minor_axis_distance: get_cap_distance(key, &term),
                index: i,
                value: key.clone()
            };
            if !matches.contains(&found) {
                matches.insert(found);
            }
        }
    }

    //next match by case-invariant contains match weighted by how much it matched
    for (i, key) in search_keys.iter().enumerate() {
        if key.to_lowercase().contains(&term.to_lowercase()) {
            let len = key.len();
            let distance = (len as i32 - term.len() as i32).abs();
            if distance as f32 <= len as f32 - (len as f32 * threshold) {
                let found = StringMatch{
                    major_axis_distance: 3,
                    minor_axis_distance: distance as u32,
                    index: i,
                    value: key.clone()
                };
                if !matches.contains(&found) {
                    matches.insert(found);
                }
            }
        }
    }
    
    //next match by levenshtien distance
    for (i, key) in search_keys.iter().enumerate() {
        let distance = get_levenshtein_distance(key, &term);
        let len = key.len();
        if distance as f32 <= len as f32 - (len as f32 * threshold) {
            let found = StringMatch{
                major_axis_distance: 4,
                minor_axis_distance: distance,
                index: i,
                value: key.clone()
            };
            if !matches.contains(&found) {
                matches.insert(found);
            }
        }
    }

    let mut sorted_matches: Vec<StringMatch> = matches.iter().cloned().collect();
    sorted_matches.sort_by(|x,y|{
        if x.major_axis_distance > y.major_axis_distance {
            return Ordering::Greater;
        } else if x.major_axis_distance < y.major_axis_distance {
            return Ordering::Less;
        } else {
            if x.minor_axis_distance > y.minor_axis_distance {
                return Ordering::Greater;
            } else if x.minor_axis_distance < y.minor_axis_distance {
                return Ordering::Less;
            } else {
                return Ordering::Equal;
            }
        }
    });

    sorted_matches
        .iter()
        .map(|m| MatchIndex(m.index, m.value.clone()))
        .collect::<Vec<MatchIndex>>()
}


#[cfg(test)]
mod fuzzymatch_tests {
    use super::*;

    macro_rules! vec_of_strings {
        // match a list of expressions separated by comma:
        ($($str:expr),*) => ({
            // create a Vec with this list of expressions,
            // calling String::from on each:
            vec![$(String::from($str),)*] as Vec<String>
        });
    }

    #[test]
    fn split_case_should_split_on_camel_case(){
        assert_eq!(vec!["a", "Happy", "Day"], split_case("aHappyDay"));
    }

    #[test]
    fn split_case_should_split_on_title_case(){
        assert_eq!(vec!["A", "Happy", "Day"], split_case("AHappyDay"));
    }

    #[test]
    fn get_levenshtein_distance_should_get_correct_distance() {
        assert_eq!(get_levenshtein_distance("x", "x"), 0);
        assert_eq!(get_levenshtein_distance("x", "y"), 1);
        assert_eq!(get_levenshtein_distance("", "x"), 1);
        assert_eq!(get_levenshtein_distance("y", ""), 1);
        assert_eq!(get_levenshtein_distance("kitten", "mutton"), 3);
        assert_eq!(get_levenshtein_distance("abc", "abbc"), 1);
        assert_eq!(get_levenshtein_distance("book", "back"), 2);
    }

    #[test]
    fn get_levenshtein_distance_should_be_case_insensitive() {
        assert_eq!(get_levenshtein_distance("KITteN", "mUttoN"), 3);
    }


    #[test]
    fn fuzzymatch_should_find_exact_match(){
        let words = vec_of_strings!["foo", "bar", "abc"];

        assert_eq!(vec![MatchIndex(0, String::from("foo"))], fuzzymatch(&words, String::from("foo"), 0.7));
        assert_eq!(vec![MatchIndex(1, String::from("bar"))], fuzzymatch(&words, String::from("bar"), 0.7));
    }

    #[test]
    fn should_match_a_character_insertion(){
        let words = vec_of_strings!["foo", "bar", "zzz"];

        assert_eq!(vec![MatchIndex(0, String::from("foo"))], fuzzymatch(&words, String::from("foos"), 0.5));
        assert_eq!(vec![MatchIndex(1, String::from("bar"))], fuzzymatch(&words, String::from("bars"), 0.5));
        assert_eq!(vec![MatchIndex(0, String::from("foo"))], fuzzymatch(&words, String::from("afoo"), 0.5));
        assert_eq!(vec![MatchIndex(1, String::from("bar"))], fuzzymatch(&words, String::from("abar"), 0.5));
        assert_eq!(vec![MatchIndex(0, String::from("foo"))], fuzzymatch(&words, String::from("fo.o"), 0.5));
        assert_eq!(vec![MatchIndex(1, String::from("bar"))], fuzzymatch(&words, String::from("b.ar"), 0.5));
    }

    #[test]
    fn should_match_a_character_deletion() {
        let words = vec_of_strings!["qux", "bar", "zzz"];

        assert_eq!(vec![MatchIndex(0, String::from("qux"))], fuzzymatch(&words, String::from("qu"), 0.5));
        assert_eq!(vec![MatchIndex(1, String::from("bar"))], fuzzymatch(&words, String::from("ba"), 0.5));
        assert_eq!(vec![MatchIndex(0, String::from("qux"))], fuzzymatch(&words, String::from("ux"), 0.5));
        assert_eq!(vec![MatchIndex(1, String::from("bar"))], fuzzymatch(&words, String::from("ar"), 0.5));
        assert_eq!(vec![MatchIndex(0, String::from("qux"))], fuzzymatch(&words, String::from("qx"), 0.5));
        assert_eq!(vec![MatchIndex(1, String::from("bar"))], fuzzymatch(&words, String::from("br"), 0.5));
    }

    #[test]
    fn should_match_a_character_swap() {
        let words = vec_of_strings!["qux", "bar", "zzz"];

        assert_eq!(vec![MatchIndex(0, String::from("qux"))], fuzzymatch(&words, String::from("quk"), 0.5));
        assert_eq!(vec![MatchIndex(1, String::from("bar"))], fuzzymatch(&words, String::from("bam"), 0.5));
        assert_eq!(vec![MatchIndex(0, String::from("qux"))], fuzzymatch(&words, String::from("lux"), 0.5));
        assert_eq!(vec![MatchIndex(1, String::from("bar"))], fuzzymatch(&words, String::from("car"), 0.5));
        assert_eq!(vec![MatchIndex(0, String::from("qux"))], fuzzymatch(&words, String::from("qix"), 0.5));
        assert_eq!(vec![MatchIndex(1, String::from("bar"))], fuzzymatch(&words, String::from("bor"), 0.5));
    }

    #[test]
    fn edit_distance_should_be_prioritized() {
        let words = vec_of_strings!["candyjake", "candyjane", "abc"];

        assert_eq!(vec![MatchIndex(1, String::from("candyjane")), MatchIndex(0, String::from("candyjake"))], fuzzymatch(&words, String::from("candycane"), 0.7));
    }

    #[test]
    fn should_not_match_if_under_threshold() {
        let words = vec_of_strings!["applehorse", "pearcat", "grapechicken", "abc"];

        assert_eq!(Vec::<MatchIndex>::new(), fuzzymatch(&words, String::from("applecat"), 0.8));
        assert_eq!(Vec::<MatchIndex>::new(), fuzzymatch(&words, String::from("pearchicken"), 0.8));
        assert_eq!(Vec::<MatchIndex>::new(), fuzzymatch(&words, String::from("grapehorse"), 0.8));
    }

    #[test]
    fn should_match_string_that_contains() {
        let words = vec_of_strings!["applehorse", "pearcat", "grapechicken", "abc"];

        assert_eq!(vec![MatchIndex(0, String::from("applehorse"))], fuzzymatch(&words, String::from("appleh"), 0.5));
        assert_eq!(vec![MatchIndex(1, String::from("pearcat"))], fuzzymatch(&words, String::from("pearc"), 0.5));
        assert_eq!(vec![MatchIndex(2, String::from("grapechicken"))], fuzzymatch(&words, String::from("grapec"), 0.5));
    }

    #[test]
    fn should_match_initals_with_caps() {
        let words = vec_of_strings!["Fuzzy Match", "Jungle Adventure", "Pacific Cruiseship", "Desert Airway"];

        assert_eq!(vec![MatchIndex(0, String::from("Fuzzy Match"))], fuzzymatch(&words, String::from("FM"), 0.7));
        assert_eq!(vec![MatchIndex(1, String::from("Jungle Adventure"))], fuzzymatch(&words, String::from("JA"), 0.7));
        assert_eq!(vec![MatchIndex(2, String::from("Pacific Cruiseship"))], fuzzymatch(&words, String::from("PC"), 0.7));
    }

    #[test]
    fn should_match_case_invariant_initals_with_caps() {
        let words = vec_of_strings!["fuzzy match", "jungle adventure", "pacific cruiseship", "desert airway"];

        assert_eq!(vec![MatchIndex(0, String::from("fuzzy match"))], fuzzymatch(&words, String::from("FM"), 0.7));
        assert_eq!(vec![MatchIndex(1, String::from("jungle adventure"))], fuzzymatch(&words, String::from("JA"), 0.7));
        assert_eq!(vec![MatchIndex(2, String::from("pacific cruiseship"))], fuzzymatch(&words, String::from("PC"), 0.7));
    }

    #[test]
    fn should_match_title_case_initals_with_caps() {
        let words = vec_of_strings!["FuzzyMatch", "JungleAdventure", "PacificCruiseship", "DesertAirway"];

        assert_eq!(vec![MatchIndex(0, String::from("FuzzyMatch"))], fuzzymatch(&words, String::from("FM"), 0.7));
        assert_eq!(vec![MatchIndex(1, String::from("JungleAdventure"))], fuzzymatch(&words, String::from("JA"), 0.7));
        assert_eq!(vec![MatchIndex(3, String::from("DesertAirway"))], fuzzymatch(&words, String::from("DA"), 0.7));
    }

    #[test]
    fn should_match_initals_with_lowercase() {
        let words = vec_of_strings!["Fuzzy Match", "Jungle Adventure", "Pacific Cruiseship", "Desert Airway"];

        assert_eq!(vec![MatchIndex(0, String::from("Fuzzy Match"))], fuzzymatch(&words, String::from("fm"), 0.7));
        assert_eq!(vec![MatchIndex(1, String::from("Jungle Adventure"))], fuzzymatch(&words, String::from("ja"), 0.7));
        assert_eq!(vec![MatchIndex(2, String::from("Pacific Cruiseship"))], fuzzymatch(&words, String::from("pc"), 0.7));
    }
    
    #[test]
    fn exact_match_should_only_produce_a_single_result() {
        let words = vec_of_strings!["blue", "BLUE", "bLUe"];

        assert_eq!(vec![MatchIndex(1, String::from("BLUE"))], fuzzymatch(&words, String::from("BLUE"), 0.7));
    }

    #[test]
    fn case_insensitive_match_should_prioritize_over_initials() {
        let words = vec_of_strings!["blue", "Big Lucky Umbrella", "BLu", "abc"];

        assert_eq!(vec![MatchIndex(2, String::from("BLu")), MatchIndex(1, String::from("Big Lucky Umbrella")), MatchIndex(0, String::from("blue"))] , fuzzymatch(&words, String::from("BLU"), 0.7));
    }

    #[test]
    fn intial_match_should_prioritize_over_contains() {
        let words = vec_of_strings!["BORK", "Big Orange Rat", "abc"];

        assert_eq!(vec![MatchIndex(1, String::from("Big Orange Rat")), MatchIndex(0, String::from("BORK"))], fuzzymatch(&words, String::from("BOR"), 0.7));
    }

    #[test]
    fn contains_match_should_prioritize_over_edit_distance_match() {
        let words = vec_of_strings!["BARB", "BARKBONE", "abc"];

        assert_eq!(vec![MatchIndex(1, String::from("BARKBONE")), MatchIndex(0, String::from("BARB"))], fuzzymatch(&words, String::from("bark"), 0.4));
    }

    #[test]
    fn fuzzymatch_should_fail_if_no_match(){
        let words = vec_of_strings!["apple", "pear", "banana", "orange"];
        assert_eq!(Vec::<MatchIndex>::new(), fuzzymatch(&words, String::from("melon"), 0.7));
    }
}