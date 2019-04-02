const {
    getLevenshteinDistance,
    splitCase,
    fuzzymatch,
 } = require("./fuzzymatch");

describe("splitCase", () => {
    it("should split split on camel case", () => {
        expect(splitCase("aHappyDay")).toBe(["a", "Happy", "Day"]);
    });

    it("should split on title case", () => {
        expect(splitCase("AHappyDay")).toBe(["A", "Happy", "Day"]);
    });
});

describe("getLevenshteinDistance", () => {
    it("should get correct distance", () => {
        expect(getLevenshteinDistance("x", "x")).toBe(0);
        expect(getLevenshteinDistance("x", "y")).toBe(1);
        expect(getLevenshteinDistance("", "x")).toBe(1);
        expect(getLevenshteinDistance("y", "")).toBe(1);
        expect(getLevenshteinDistance("kitten", "mutton")).toBe(3);
        expect(getLevenshteinDistance("abc", "abbc")).toBe(1);
        expect(getLevenshteinDistance("book", "back")).toBe(2);
    });
    
    it("should be case insensitive", () => {
        expect(getLevenshteinDistance("KITteN", "mUttoN")).toBe(3);
    });
});

describe("fuzzy match", () => {
    
    it("fuzzymatch should find exact match", () => {
        const words = ["foo", "bar", "abc"];

        expect(fuzzymatch(words, "foo", 0.7)).toBe([0, "foo"]);
        expect(fuzzymatch(words, "bar", 0.7)).toBe([1, "bar"]);
    });
    
    it("should match a character insertion", () => {
        const words = ["foo", "bar", "zzz"];

        expect(fuzzymatch(words, "foos", 0.5)).toBe([0, "foo"]);
        expect(fuzzymatch(words, "bars", 0.5)).toBe([1, "bar"]);
        expect(fuzzymatch(words, "afoo", 0.5)).toBe([0, "foo"]);
        expect(fuzzymatch(words, "abar", 0.5)).toBe([1, "bar"]);
        expect(fuzzymatch(words, "fo.o", 0.5)).toBe([0, "foo"]);
        expect(fuzzymatch(words, "b.ar", 0.5)).toBe([1, "bar"]);
    });
    
    it("should match a character deletion", () => {
        const words = ["qux", "bar", "zzz"];

        expect(fuzzymatch(words, "qu", 0.5)).toBe([0, "qux"]);
        expect(fuzzymatch(words, "ba", 0.5)).toBe([1, "bar"]);
        expect(fuzzymatch(words, "ux", 0.5)).toBe([0, "qux"]);
        expect(fuzzymatch(words, "ar", 0.5)).toBe([1, "bar"]);
        expect(fuzzymatch(words, "qx", 0.5)).toBe([0, "qux"]);
        expect(fuzzymatch(words, "br", 0.5)).toBe([1, "bar"]);
    });
    
    it("should match a character swap", () => {
        const words = ["qux", "bar", "zzz"];

        expect(fuzzymatch(words, "quk", 0.5)).toBe([0, "qux"]);
        expect(fuzzymatch(words, "bam", 0.5)).toBe([1, "bar"]);
        expect(fuzzymatch(words, "lux", 0.5)).toBe([0, "qux"]);
        expect(fuzzymatch(words, "car", 0.5)).toBe([1, "bar"]);
        expect(fuzzymatch(words, "qix", 0.5)).toBe([0, "qux"]);
        expect(fuzzymatch(words, "bor", 0.5)).toBe([1, "bar"]);
    });
    
    it("edit distance should be prioritized", () => {
        const words = ["candyjake", "candyjane", "abc"];

        expect(fuzzymatch(words, "candycane", 0.7)).toBe([[1, "candyjane"], [0, "candyjake"]]);
    });
    
    it("should not match if under threshold", () => {
        const words = ["applehorse", "pearcat", "grapechicken", "abc"];

        expect(fuzzymatch(words, "applecat", 0.8)).toBe([]);
        expect(fuzzymatch(words, "pearchicken", 0.8)).toBe([]);
        expect(fuzzymatch(words, "grapehorse", 0.8)).toBe([]);
    });
    
    it("should match string that contains", () => {
        const words = ["applehorse", "pearcat", "grapechicken", "abc"];

        expect(fuzzymatch(words, "appleh", 0.5)).toBe([[0, "applehorse"]]);
        expect(fuzzymatch(words, "pearc", 0.5)).toBe([[1, "pearcat"]]);
        expect(fuzzymatch(words, "grapec", 0.5)).toBe([[2, "grapechicken"]]);
    });
    
    it("should match initals with caps", () => {
        const words = [
            "Fuzzy Match",
            "Jungle Adventure",
            "Pacific Cruiseship",
            "Desert Airway",
        ];

        expect(fuzzymatch(words, "FM", 0.7)).toBe([[0, "Fuzzy Match"]]);
        expect(fuzzymatch(words, "JA", 0.7)).toBe([[1, "Jungle Adventure"]]);
        expect(fuzzymatch(words, "PC", 0.7)).toBe([[2, "Pacific Cruiseship"]]);
    });
    
    it("should match case invariant initals with caps", () => {
        const words = [
            "fuzzy match",
            "jungle adventure",
            "pacific cruiseship",
            "desert airway",
        ];

        expect(fuzzymatch(words, "FM", 0.7)).toBe([[0, "fuzzy match"]]);
        expect(fuzzymatch(words, "JA", 0.7)).toBe([[1, "jungle adventure"]]);
        expect(fuzzymatch(words, "PC", 0.7)).toBe([[2, "pacific cruiseship"]]);
    });

    it("should match kebab initials", () => {
        const words = [
            "fuzzy-match",
            "jungle-adventure",
            "pacific-cruiseship",
            "desert-airway",
        ];

        expect(fuzzymatch(words, "FM", 0.7)).toBe([[0, "fuzzy-match"]]);
        expect(fuzzymatch(words, "JA", 0.7)).toBe([[1, "jungle-adventure"]]);
        expect(fuzzymatch(words, "PC", 0.7)).toBe([[2, "pacific-cruiseship"]]);
    });

    it("should_match_snake_initials()", () => {
        const words = [
            "fuzzy_match",
            "jungle_adventure",
            "pacific_cruiseship",
            "desert_airway",
        ];

        expect(fuzzymatch(words, "FM", 0.7)).toBe([[0, "fuzzy_match"]]);
        expect(fuzzymatch(words, "JA", 0.7)).toBe([[1, "jungle_adventure"]]);
        expect(fuzzymatch(words, "PC", 0.7)).toBe([[2, "pacific_cruiseship"]]);
    });
    
    it("should match title case initals with caps", () => {
        const words = [
            "FuzzyMatch",
            "JungleAdventure",
            "PacificCruiseship",
            "DesertAirway",
        ];

        expect(fuzzymatch(words, "FM", 0.7)).toBe([[0, "FuzzyMatch"]]);
        expect(fuzzymatch(words, "JA", 0.7)).toBe([[1, "JungleAdventure"]]);
        expect(fuzzymatch(words, "DA", 0.7)).toBe([[3, "DesertAirway"]]);
    });
    
    it("should match initals with lowercase", () => {
        const words = [
            "Fuzzy Match",
            "Jungle Adventure",
            "Pacific Cruiseship",
            "Desert Airway",
        ];

        expect(fuzzymatch(words, "fm", 0.7)).toBe([[0, "Fuzzy Match"]]);
        expect(fuzzymatch(words, "ja", 0.7)).toBe([[1, "Jungle Adventure"]]);
        expect(fuzzymatch(words, "pc", 0.7)).toBe([[2, "Pacific Cruiseship"]]);
    });
    
    it("exact match should only produce a single result", () => {
        const words = ["blue", "BLUE", "bLUe"];

        expect(fuzzymatch(words, "BLUE", 0.7)).toBe([[1, "BLUE"]]);
    });
    
    it("case insensitive match should prioritize over initials", () => {
        const words = ["blue", "Big Lucky Umbrella", "BLu", "abc"];

        expect(fuzzymatch(words, "BLU", 0.7)).toBe([
            [2, "BLu"],
            [1, "Big Lucky Umbrella"],
            [0, "blue"]
        ]);
    });
    
    it("intial match should prioritize over contains", () => {
        const words = ["BORK", "Big Orange Rat", "abc"];

        expect(fuzzymatch(words, "BOR", 0.7), [[1, "Big Orange Rat"], [0, "BORK"]]);
    });
    
    it("contains match should prioritize over edit distance match", () => {
        const words = ["BARB", "BARKBONE", "abc"];

        expect(fuzzymatch(words, "bark", 0.4)).toBe([[1, "BARKBONE"], [0, "BARB"]]);
    });
    
    it("fuzzymatch should return no match if empty", () => {
        const words = [];

        expect(fuzzymatch(words, "any", 0.7)).toBe([]);
    });

    
    it("fuzzymatch should return no match if no term", () => {
        const words = ["apple", "pear", "banana", "orange"];

        expect(fuzzymatch(words, "", 0.7)).toBe([]);
    });

    
    it("fuzzymatch_should_fail_if_no_match", () => {
        const words = ["apple", "pear", "banana", "orange"];

        expect(fuzzymatch(words, "melon", 0.7)).toBe([]);
    });
});