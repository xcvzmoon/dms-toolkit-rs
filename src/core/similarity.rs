// core/similarity.rs
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub enum SimilarityMethod {
    Jaccard,
    Ngram,
    Levenshtein,
    Hybrid,
}

/// Fast pre-filtering using length difference heuristic
pub fn pre_filter_by_length(source: &str, target: &str, threshold: f64) -> bool {
    let difference = (source.len() as i64 - target.len() as i64).abs() as f64;
    let max = source.len().max(target.len()) as f64;
    if max == 0.0 {
        return true;
    }
    (difference / max) * 100.0 <= (100.0 - threshold)
}

/// Jaccard similarity (word-based, very fast)
pub fn jaccard_similarity(source: &str, target: &str) -> f64 {
    let source_words: HashSet<String> = source
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect();
    let target_words: HashSet<String> = target
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect();

    let intersection_size = source_words.intersection(&target_words).count();
    let union_size = source_words.union(&target_words).count();

    if union_size == 0 {
        return 0.0;
    }

    (intersection_size as f64 / union_size as f64) * 100.0
}

/// N-gram similarity with configurable n
pub fn ngram_similarity(source: &str, target: &str, n: usize) -> f64 {
    fn get_ngrams(text: &str, n: usize) -> HashSet<String> {
        let cleaned: String = text
            .to_lowercase()
            .chars()
            .filter(|c| !c.is_whitespace() || *c == ' ')
            .collect();

        let cleaned = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");

        if cleaned.len() < n {
            return HashSet::new();
        }

        cleaned
            .chars()
            .collect::<Vec<_>>()
            .windows(n)
            .map(|window| window.iter().collect::<String>())
            .collect()
    }

    let source_ngrams = get_ngrams(source, n);
    let target_ngrams = get_ngrams(target, n);

    let intersection_size = source_ngrams.intersection(&target_ngrams).count();
    let union_size = source_ngrams.union(&target_ngrams).count();

    if union_size == 0 {
        return 0.0;
    }

    (intersection_size as f64 / union_size as f64) * 100.0
}

/// Optimized Levenshtein distance with early termination
pub fn levenshtein_distance(source: &str, target: &str, max_distance: Option<usize>) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    if source_chars.is_empty() {
        return target_chars.len();
    }
    if target_chars.is_empty() {
        return source_chars.len();
    }

    let source_len = source_chars.len();
    let target_len = target_chars.len();

    // Use shorter string as rows for memory efficiency
    let (rows, cols, use_swap) = if source_len < target_len {
        (source_len + 1, target_len + 1, false)
    } else {
        (target_len + 1, source_len + 1, true)
    };

    let (s_chars, t_chars) = if use_swap {
        (&target_chars, &source_chars)
    } else {
        (&source_chars, &target_chars)
    };

    let mut previous: Vec<usize> = (0..cols).collect();
    let mut current: Vec<usize> = vec![0; cols];

    for i in 1..rows {
        current[0] = i;
        let mut row_min = i;

        for j in 1..cols {
            let cost = if s_chars[i - 1] == t_chars[j - 1] {
                0
            } else {
                1
            };
            current[j] = (current[j - 1] + 1)
                .min(previous[j] + 1)
                .min(previous[j - 1] + cost);

            row_min = row_min.min(current[j]);
        }

        // Early termination if this row exceeds max_distance
        if let Some(max_dist) = max_distance {
            if row_min > max_dist {
                return max_dist + 1;
            }
        }

        std::mem::swap(&mut previous, &mut current);
    }

    previous[cols - 1]
}

/// Levenshtein similarity as percentage
pub fn levenshtein_similarity(source: &str, target: &str, max_distance: Option<usize>) -> f64 {
    let max_length = source.len().max(target.len());
    if max_length == 0 {
        return 100.0;
    }

    let distance = levenshtein_distance(source, target, max_distance);

    if let Some(max_dist) = max_distance {
        if distance > max_dist {
            return 0.0;
        }
    }

    ((max_length - distance) as f64 / max_length as f64) * 100.0
}

/// Hybrid similarity with progressive filtering (matches your TypeScript logic)
pub fn hybrid_similarity(source: &str, target: &str) -> f64 {
    // Fast initial filter using Jaccard
    let jaccard_score = jaccard_similarity(source, target);

    if jaccard_score < 20.0 {
        return jaccard_score;
    }

    // For small texts, use Levenshtein with early termination
    if source.len() < 1000 && target.len() < 1000 {
        let max_length = source.len().max(target.len());
        let max_allowed_distance = (max_length as f64 * 0.8) as usize;

        let distance = levenshtein_distance(source, target, Some(max_allowed_distance));

        if distance > max_allowed_distance {
            return 20.0;
        }

        return ((max_length - distance) as f64 / max_length as f64) * 100.0;
    }

    // For larger texts, use N-gram
    ngram_similarity(source, target, 3)
}

/// Calculate similarity based on method
pub fn calculate_similarity(source: &str, target: &str, method: SimilarityMethod) -> f64 {
    match method {
        SimilarityMethod::Jaccard => jaccard_similarity(source, target),
        SimilarityMethod::Ngram => ngram_similarity(source, target, 3),
        SimilarityMethod::Levenshtein => levenshtein_similarity(source, target, None),
        SimilarityMethod::Hybrid => hybrid_similarity(source, target),
    }
}

/// Compare one text against multiple documents in parallel
pub fn compare_with_documents(
    source_text: &str,
    target_texts: &[String],
    method: SimilarityMethod,
    threshold: f64,
) -> Vec<(usize, f64)> {
    target_texts
        .par_iter()
        .enumerate()
        .filter_map(|(idx, target)| {
            // Pre-filter by length
            if !pre_filter_by_length(source_text, target, threshold) {
                return None;
            }

            let similarity = calculate_similarity(source_text, target, method);

            if similarity >= threshold {
                Some((idx, similarity))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaccard_similarity() {
        let text1 = "hello world";
        let text2 = "hello there world";
        let score = jaccard_similarity(text1, text2);
        assert!(score > 0.0 && score < 100.0);
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting", None), 3);
        assert_eq!(levenshtein_distance("", "abc", None), 3);
        assert_eq!(levenshtein_distance("abc", "abc", None), 0);
    }

    #[test]
    fn test_pre_filter() {
        assert!(pre_filter_by_length("hello", "hello world", 30.0));
        assert!(!pre_filter_by_length(
            "a",
            "this is a very long string",
            30.0
        ));
    }
}
