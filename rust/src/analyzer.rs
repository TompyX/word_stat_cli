use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
struct AnalysisResult {
    word_count: usize,
    char_count: usize,
    letter_count: usize,
    sentence_count: usize,
    longest_word: Option<String>,
    longest_word_length: Option<usize>,
    most_common_word: Option<String>,
    most_common_word_count: Option<usize>,
    search_word: Option<String>,
    search_word_count: Option<usize>,
    char_freq: HashMap<char, usize>,
    avg_word_length: f64,
}

pub fn analyze(content: &str, search: Option<&str>, json_output: bool) {
    let word_count = content.split_whitespace().count();
    let char_count = content.chars().count();
    let letter_count = content.chars().filter(|c| !c.is_whitespace()).count();
    let sentence_count = content.matches('.').count()
        + content.matches('!').count()
        + content.matches('?').count();

    let longest_word = content
        .split_whitespace()
        .max_by_key(|word| word.chars().count());
    let longest_word_length = longest_word.map(|w| w.chars().count());

    let mut freq: HashMap<String, usize> = HashMap::new();
    for word in content.split_whitespace().map(|w| w.to_lowercase()) {
        *freq.entry(word).or_insert(0) += 1;
    }
    let (most_common_word, most_common_word_count) = freq.iter()
        .max_by_key(|entry| entry.1)
        .map(|(w, c)| (Some(w.clone()), Some(*c)))
        .unwrap_or((None, None));

    let (search_word, search_word_count) = if let Some(search_word) = search {
        let search_word_lc = search_word.to_lowercase();
        let count = content
            .split_whitespace()
            .filter(|w| w.to_lowercase() == search_word_lc)
            .count();
        (Some(search_word_lc), Some(count))
    } else {
        (None, None)
    };

    let mut char_freq: HashMap<char, usize> = HashMap::new();
    for c in content.chars().filter(|c| c.is_alphabetic()) {
        *char_freq.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }

    let words: Vec<&str> = content.split_whitespace().collect();
    let avg_word_length = if !words.is_empty() {
        let total_length: usize = words.iter().map(|w| w.chars().count()).sum();
        total_length as f64 / words.len() as f64
    } else {
        0.0
    };

    let result = AnalysisResult {
        word_count,
        char_count,
        letter_count,
        sentence_count,
        longest_word: longest_word.map(|s| s.to_string()),
        longest_word_length,
        most_common_word,
        most_common_word_count,
        search_word,
        search_word_count,
        char_freq,
        avg_word_length,
    };

    if json_output {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        println!("A szavak száma: {}", word_count);
        println!("A karakterek száma: {}", char_count);
        println!("A betűk száma (szóköz nélkül): {}", letter_count);
        println!("A mondatok száma (pont, !, ? alapján): {}", sentence_count);
        match &result.longest_word {
            Some(word) => println!("A leghosszabb szó: \"{}\" ({} karakter)", word, result.longest_word_length.unwrap()),
            None => println!("Nincs szó a szövegben."),
        }
        match (&result.most_common_word, &result.most_common_word_count) {
            (Some(word), Some(count)) => println!("A leggyakoribb szó: \"{}\" ({} alkalommal)", word, count),
            _ => println!("Nincs szó a szövegben."),
        }
        if let (Some(word), Some(count)) = (&result.search_word, &result.search_word_count) {
            println!("A(z) \"{}\" szó előfordulása: {} alkalommal", word, count);
        }
        println!("Betűgyakoriság:");
        for (ch, count) in &result.char_freq {
            println!("  '{}': {}", ch, count);
        }
        println!("Átlagos szóhossz: {:.2}", avg_word_length);
    }
}