//! Text similarity comparison algorithms for document matching.
//!
//! This module provides multiple algorithms for comparing text similarity,
//! used by the `process_and_compare_files` function to match extracted
//! text against reference documents.

use rayon::prelude::*;
use std::collections::HashSet;

/// Enumeration of available similarity calculation methods.
///
/// Each method has different characteristics in terms of speed and accuracy,
/// making them suitable for different use cases.
#[derive(Debug, Clone, Copy)]
pub enum SimilarityMethod {
    /// Fast word-based similarity using Jaccard index.
    ///
    /// Best for quick comparisons and initial filtering. Splits texts into
    /// words (lowercased) and calculates intersection over union of word sets.
    /// Very fast but may miss character-level similarities.
    Jaccard,

    /// Character n-gram based similarity (uses 3-grams).
    ///
    /// Good for longer texts where word-based methods might miss character-level
    /// similarities. Breaks texts into character sequences and compares shared n-grams.
    Ngram,

    /// Edit distance based similarity using Levenshtein distance.
    ///
    /// Calculates the minimum number of edits (insertions, deletions, substitutions)
    /// needed to transform one string into another. More accurate but slower for
    /// long texts. Converts edit distance to similarity percentage.
    Levenshtein,

    /// Progressive filtering approach combining multiple methods.
    ///
    /// Balances speed and accuracy by:
    /// 1. Fast Jaccard check - if score < 20%, return immediately
    /// 2. For small texts (< 1000 chars): Use Levenshtein with early termination
    /// 3. For larger texts: Use N-gram similarity
    ///
    /// This is the default method and recommended for most use cases.
    Hybrid,
}

/// Fast pre-filtering using length difference heuristic.
///
/// This function quickly filters out obviously dissimilar texts by comparing
/// their lengths. If the length difference is too large relative to the threshold,
/// the texts are unlikely to be similar enough to warrant expensive similarity
/// calculations.
///
/// # Arguments
///
/// * `source` - The source text to compare
/// * `target` - The target text to compare against
/// * `threshold` - The similarity threshold percentage (0-100)
///
/// # Returns
///
/// `true` if the texts pass the length-based pre-filter (should proceed with
/// similarity calculation), `false` if they should be filtered out.
///
/// # Algorithm
///
/// Calculates the relative length difference: `|source_len - target_len| / max_len * 100`
/// If this difference is greater than `(100 - threshold)`, the texts are filtered out.
///
/// # Example
///
/// ```
/// # use dms_toolkit_rs::core::similarity::pre_filter_by_length;
/// // Similar length texts pass the filter
/// assert!(pre_filter_by_length("hello world", "hello there", 30.0));
///
/// // Very different lengths are filtered out
/// assert!(!pre_filter_by_length("a", "this is a very long string", 30.0));
/// ```
pub fn pre_filter_by_length(source: &str, target: &str, threshold: f64) -> bool {
    let difference = (source.len() as i64 - target.len() as i64).abs() as f64;
    let max = source.len().max(target.len()) as f64;
    if max == 0.0 {
        return true;
    }
    (difference / max) * 100.0 <= (100.0 - threshold)
}

/// Calculates Jaccard similarity between two texts (word-based).
///
/// Jaccard similarity is a fast word-based similarity metric that compares
/// the overlap of word sets between two texts. It's very efficient and good
/// for initial filtering or quick comparisons.
///
/// # Algorithm
///
/// 1. Splits both texts into words (whitespace-separated)
/// 2. Converts words to lowercase for case-insensitive comparison
/// 3. Creates sets of unique words for each text
/// 4. Calculates: `intersection_size / union_size * 100`
///
/// # Arguments
///
/// * `source` - The source text to compare
/// * `target` - The target text to compare against
///
/// # Returns
///
/// Similarity percentage (0.0 to 100.0), where:
/// - 100.0 means identical word sets
/// - 0.0 means no shared words
///
/// # Performance
///
/// Very fast - O(n + m) where n and m are the number of words in each text.
/// Best suited for quick filtering or when word-level similarity is sufficient.
///
/// # Example
///
/// ```
/// # use dms_toolkit_rs::core::similarity::jaccard_similarity;
/// let text1 = "hello world";
/// let text2 = "hello there world";
/// let similarity = jaccard_similarity(text1, text2);
/// // Returns a value between 0 and 100 based on shared words
/// ```
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

/// Calculates n-gram similarity between two texts.
///
/// N-gram similarity compares texts at the character level by breaking them
/// into sequences of n consecutive characters (n-grams) and comparing the
/// overlap of these sequences. This method is good for capturing character-level
/// similarities that word-based methods might miss.
///
/// # Algorithm
///
/// 1. Normalizes texts: converts to lowercase and normalizes whitespace
/// 2. Generates n-grams (character sequences of length n) for both texts
/// 3. Creates sets of unique n-grams
/// 4. Calculates: `intersection_size / union_size * 100`
///
/// # Arguments
///
/// * `source` - The source text to compare
/// * `target` - The target text to compare against
/// * `n` - The n-gram size (typically 2-4, commonly 3 for trigrams)
///
/// # Returns
///
/// Similarity percentage (0.0 to 100.0), where:
/// - 100.0 means identical n-gram sets
/// - 0.0 means no shared n-grams
///
/// # Performance
///
/// Moderate speed - O(n + m) where n and m are text lengths. Good balance
/// between accuracy and performance for longer texts.
///
/// # Use Cases
///
/// - Longer texts where word-based methods might miss similarities
/// - Texts with typos or variations in word boundaries
/// - When character-level similarity is important
///
/// # Example
///
/// ```
/// # use dms_toolkit_rs::core::similarity::ngram_similarity;
/// let text1 = "hello world";
/// let text2 = "hello world!";
/// let similarity = ngram_similarity(text1, text2, 3); // Uses trigrams
/// ```
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

/// Calculates Levenshtein distance (edit distance) between two strings.
///
/// Levenshtein distance is the minimum number of single-character edits
/// (insertions, deletions, or substitutions) required to transform one string
/// into another. This function uses an optimized implementation with early
/// termination for better performance.
///
/// # Algorithm
///
/// Uses dynamic programming with space optimization:
/// - Uses only two rows instead of a full matrix (O(min(m,n)) space)
/// - Swaps shorter string as rows for memory efficiency
/// - Supports early termination if distance exceeds `max_distance`
///
/// # Arguments
///
/// * `source` - The source string
/// * `target` - The target string
/// * `max_distance` - Optional maximum distance threshold for early termination.
///   If the distance exceeds this value, the function returns `max_distance + 1`
///   immediately without completing the calculation.
///
/// # Returns
///
/// The Levenshtein distance (number of edits), or `max_distance + 1` if the
/// distance exceeds the threshold.
///
/// # Performance
///
/// Time complexity: O(m * n) where m and n are string lengths.
/// Space complexity: O(min(m, n)) due to space optimization.
///
/// Early termination significantly improves performance when comparing
/// obviously dissimilar strings.
///
/// # Example
///
/// ```
/// # use dms_toolkit_rs::core::similarity::levenshtein_distance;
/// assert_eq!(levenshtein_distance("kitten", "sitting", None), 3);
/// assert_eq!(levenshtein_distance("", "abc", None), 3);
/// assert_eq!(levenshtein_distance("abc", "abc", None), 0);
///
/// // Early termination example
/// let distance = levenshtein_distance("short", "very long string", Some(5));
/// assert!(distance > 5); // Returns early
/// ```
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

/// Calculates Levenshtein similarity as a percentage.
///
/// Converts Levenshtein distance into a similarity percentage by comparing
/// the edit distance to the maximum possible distance (the length of the
/// longer string).
///
/// # Formula
///
/// `similarity = ((max_length - distance) / max_length) * 100`
///
/// # Arguments
///
/// * `source` - The source string
/// * `target` - The target string
/// * `max_distance` - Optional maximum distance threshold. If the distance
///   exceeds this value, returns 0.0 immediately.
///
/// # Returns
///
/// Similarity percentage (0.0 to 100.0), where:
/// - 100.0 means identical strings (distance = 0)
/// - 0.0 means maximum distance or distance exceeds threshold
///
/// # Example
///
/// ```
/// # use dms_toolkit_rs::core::similarity::levenshtein_similarity;
/// // Identical strings
/// assert_eq!(levenshtein_similarity("hello", "hello", None), 100.0);
///
/// // Similar strings
/// let similarity = levenshtein_similarity("kitten", "sitting", None);
/// // Returns a value between 0 and 100 based on edit distance
/// ```
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

/// Calculates hybrid similarity using progressive filtering.
///
/// This method combines multiple similarity algorithms in a progressive
/// filtering approach to balance speed and accuracy. It's the default
/// method and recommended for most use cases.
///
/// # Algorithm
///
/// 1. **Fast Jaccard Check**: First performs a fast word-based Jaccard
///    similarity check. If the score is below 20%, returns immediately
///    (texts are too dissimilar).
///
/// 2. **Small Text Handling** (< 1000 characters):
///    - Uses Levenshtein distance with early termination
///    - Calculates maximum allowed distance as 80% of max length
///    - If distance exceeds threshold, returns 20.0 (low similarity)
///    - Otherwise converts distance to similarity percentage
///
/// 3. **Large Text Handling** (>= 1000 characters):
///    - Uses N-gram similarity with 3-grams (trigrams)
///    - More efficient than Levenshtein for long texts
///    - Captures character-level similarities
///
/// # Arguments
///
/// * `source` - The source text to compare
/// * `target` - The target text to compare against
///
/// # Returns
///
/// Similarity percentage (0.0 to 100.0)
///
/// # Performance Characteristics
///
/// - Very fast for dissimilar texts (early Jaccard exit)
/// - Accurate for small texts (Levenshtein)
/// - Efficient for large texts (N-gram)
/// - Best overall balance of speed and accuracy
///
/// # Example
///
/// ```
/// # use dms_toolkit_rs::core::similarity::hybrid_similarity;
/// let text1 = "The quick brown fox jumps over the lazy dog";
/// let text2 = "The quick brown fox jumps over the lazy dog";
/// let similarity = hybrid_similarity(text1, text2);
/// // Returns 100.0 for identical texts
/// ```
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

/// Calculates similarity between two texts using the specified method.
///
/// This is a dispatcher function that routes to the appropriate similarity
/// algorithm based on the `SimilarityMethod` enum value.
///
/// # Arguments
///
/// * `source` - The source text to compare
/// * `target` - The target text to compare against
/// * `method` - The similarity method to use (Jaccard, Ngram, Levenshtein, or Hybrid)
///
/// # Returns
///
/// Similarity percentage (0.0 to 100.0) calculated using the specified method
///
/// # Example
///
/// ```
/// # use dms_toolkit_rs::core::similarity::{calculate_similarity, SimilarityMethod};
/// let text1 = "hello world";
/// let text2 = "hello there";
///
/// let jaccard = calculate_similarity(text1, text2, SimilarityMethod::Jaccard);
/// let hybrid = calculate_similarity(text1, text2, SimilarityMethod::Hybrid);
/// ```
pub fn calculate_similarity(source: &str, target: &str, method: SimilarityMethod) -> f64 {
    match method {
        SimilarityMethod::Jaccard => jaccard_similarity(source, target),
        SimilarityMethod::Ngram => ngram_similarity(source, target, 3),
        SimilarityMethod::Levenshtein => levenshtein_similarity(source, target, None),
        SimilarityMethod::Hybrid => hybrid_similarity(source, target),
    }
}

/// Compares one text against multiple reference texts in parallel.
///
/// This function is the main entry point for similarity comparison. It takes
/// a source text and compares it against multiple reference texts using the
/// specified similarity method, returning only matches above the threshold.
///
/// # Processing Flow
///
/// 1. **Parallel Iteration**: Uses Rayon to process all reference texts in parallel
/// 2. **Pre-filtering**: Applies length-based pre-filtering to quickly eliminate
///    obviously dissimilar texts before expensive calculations
/// 3. **Similarity Calculation**: Calculates similarity using the specified method
/// 4. **Threshold Filtering**: Only includes matches with similarity >= threshold
/// 5. **Result Collection**: Returns pairs of (reference_index, similarity_percentage)
///
/// # Arguments
///
/// * `source_text` - The text extracted from a file to compare
/// * `target_texts` - A slice of reference text strings to compare against
/// * `method` - The similarity method to use (Jaccard, Ngram, Levenshtein, or Hybrid)
/// * `threshold` - The minimum similarity percentage (0-100) required for a match
///
/// # Returns
///
/// A vector of tuples `(usize, f64)` where:
/// - `usize` is the index of the reference text in the input array
/// - `f64` is the similarity percentage (0-100)
///
/// Only matches with similarity >= threshold are included. Results are not
/// guaranteed to be in any particular order due to parallel processing.
///
/// # Performance
///
/// - Parallel processing: All comparisons run simultaneously across CPU cores
/// - Pre-filtering: Quickly eliminates dissimilar texts before expensive calculations
/// - Early termination: Some methods (like Levenshtein) support early termination
///
/// # Example
///
/// ```
/// # use dms_toolkit_rs::core::similarity::{compare_with_documents, SimilarityMethod};
/// let source = "The quick brown fox";
/// let references = vec![
///     "The quick brown fox jumps".to_string(),
///     "A completely different text".to_string(),
///     "The quick brown fox".to_string(),
/// ];
///
/// let matches = compare_with_documents(
///     source,
///     &references,
///     SimilarityMethod::Hybrid,
///     50.0, // 50% threshold
/// );
///
/// // matches contains (index, similarity) pairs for texts above 50% similarity
/// ```
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
