use std::collections::{HashMap, HashSet};

fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    // Convert banned to HashSet for O(1) lookup
    let banned_set: HashSet<String> = banned
        .into_iter()
        .map(|word| word.to_lowercase())
        .collect();

    // HashMap to count word frequencies
    let mut word_count: HashMap<String, i32> = HashMap::new();

    // Extract words and count them
    let words = paragraph
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty());

    for word in words {
        let word_lower = word.to_lowercase();

        // Skip if banned
        if banned_set.contains(&word_lower) {
            continue;
        }

        // Use or_insert with dereference to increment count
        let count = word_count.entry(word_lower).or_insert(0);
        *count += 1;
    }

    // Find the word with maximum count
    word_count
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(word, _)| word)
        .unwrap_or_default()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            most_common_word(
                String::from("Bob hit a ball, the hit BALL flew far after it was hit."),
                vec![String::from("hit")]
            ),
            String::from("ball"),
        );

        assert_eq!(
            most_common_word(
                String::from("a, a, a, a, b,b,b,c, c"),
                vec![String::from("a")]
            ),
            String::from("b"),
        );
    }
}