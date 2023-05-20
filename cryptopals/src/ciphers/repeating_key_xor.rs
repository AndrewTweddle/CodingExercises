use thiserror::Error;
use super::common::score_bytes_as_ascii_message;

pub fn encrypt(input: &str, key: &str) -> Vec<u8> {
    let key_iter = key.as_bytes().iter().cycle();
    input
        .as_bytes()
        .iter()
        .zip(key_iter)
        .map(|(ch, key_char)| ch ^ key_char)
        .collect::<Vec<u8>>()
}

pub fn get_best_key_sizes_and_likely_decryptions(
    encrypted_bytes: &Vec<u8>,
    mut max_key_size: usize,
    num_hamming_distance_samples: usize,
    num_candidate_decryptions: usize,
) -> Vec<(usize, String)> {
    if max_key_size > encrypted_bytes.len() {
        max_key_size = encrypted_bytes.len();
    }
    let mut key_sizes_and_scores: Vec<(usize, f32)> = (2..=max_key_size)
        .map(|key_size| {
            (
                key_size,
                get_mean_normalized_hamming_distance_score_for_key_size(
                    encrypted_bytes.as_slice(),
                    key_size,
                    num_hamming_distance_samples,
                )
                    .unwrap(),
            )
        })
        .collect();

    key_sizes_and_scores.sort_by(|&(_, score1), &(_, score2)| score1.partial_cmp(&score2).unwrap());

    key_sizes_and_scores
        .iter()
        .take(num_candidate_decryptions)
        .map(|(key_size, _)| {
            let decrypted_bytes =
                decrypt_given_key_size(&encrypted_bytes, *key_size);

            (
                *key_size,
                String::from_utf8_lossy(decrypted_bytes.as_slice()).into_owned(),
            )
        })
        .collect::<Vec<(usize, String)>>()
}

pub fn decrypt_given_key_size(
    encrypted_bytes: &[u8],
    key_size: usize,
) -> Vec<u8> {
    // arrange into a matrix of width key_size, with each column a Vec<u8>
    let capacity = (encrypted_bytes.len() + key_size - 1) / key_size;
    let mut bytes_in_columns = vec![Vec::<u8>::with_capacity(capacity); key_size as usize];
    for chunk in encrypted_bytes.chunks(key_size) {
        for col in 0..chunk.len() {
            bytes_in_columns[col].push(chunk[col])
        }
    }

    let decrypted_columns: Vec<Vec<u8>> = bytes_in_columns
        .iter()
        .map(|column| {
            let decrypted_column = (0_u8..255)
                .map(|key| {
                    let mapped_column = column
                        .iter()
                        .map(|&col_val| col_val ^ key)
                        .collect::<Vec<u8>>();
                    let score = score_bytes_as_ascii_message(mapped_column.as_slice());
                    (mapped_column, score)
                })
                .min_by_key(|(_, score)| -*score)
                .map(|(candidate_column, _)| candidate_column)
                .unwrap();

            decrypted_column
        })
        .collect();

    // Now read off the rows from the given columns...
    let mut plain_bytes = Vec::<u8>::with_capacity(encrypted_bytes.len());
    for row in 0..capacity {
        for column in &decrypted_columns {
            if column.len() > row {
                plain_bytes.push(column[row]);
            }
        }
    }

    plain_bytes
}

pub fn hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> usize {
    bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(byte1, byte2)| {
            let mut diff_byte = byte1 ^ byte2;
            let mut differing_bits_count: usize = 0;
            for _ in 0..8 {
                if diff_byte % 2 == 1 {
                    differing_bits_count += 1
                };
                diff_byte >>= 1;
            }
            differing_bits_count
        })
        .sum::<usize>()
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MeanNormalizedHammingDistanceScoreError {
    #[error("At least 2 samples are required to get a normalized score")]
    TooFewSamples,

    #[error("Key size ({key_size}) and number of samples ({num_samples}) \
            too large for cipher length ({cipher_len})")]
    KeySizeAndSamplesTooLargeForCipherLength {
        cipher_len: usize,
        key_size: usize,
        num_samples: usize,
    },
}

pub fn get_mean_normalized_hamming_distance_score_for_key_size(
    cipher_bytes: &[u8],
    key_size: usize,
    num_samples: usize,
) -> Result<f32, MeanNormalizedHammingDistanceScoreError> {
    if num_samples < 2 {
        return Err(MeanNormalizedHammingDistanceScoreError::TooFewSamples);
    }
    if cipher_bytes.len() < num_samples * key_size {
        return Err(
            MeanNormalizedHammingDistanceScoreError::KeySizeAndSamplesTooLargeForCipherLength {
                cipher_len: cipher_bytes.len(),
                num_samples,
                key_size
            });
    }
    let num_pairs_of_samples = num_samples * (num_samples + 1) / 2;
    let mut hamming_distances: Vec<usize> = Vec::with_capacity(num_pairs_of_samples);
    for i in 0..(num_samples - 1) {
        let sample1_start = i * key_size;
        let sample1_end = (i + 1) * key_size;
        let sample1 = &cipher_bytes[sample1_start..sample1_end];
        for j in (i + 1)..num_samples {
            let sample2_start = j * key_size;
            let sample2_end = (j + 1) * key_size;
            let sample2 = &cipher_bytes[sample2_start..sample2_end];
            let dist = hamming_distance(sample1, sample2);
            hamming_distances.push(dist);
        }
    }
    let sum_of_distances: usize = hamming_distances.iter().sum::<usize>();
    let avg_distance = (sum_of_distances as f32) / (hamming_distances.len() as f32);
    Ok(avg_distance / (key_size as f32))
}

#[cfg(test)]
mod tests {
    use super::hamming_distance;

    fn hamming_distance_of_strs(s1: &str, s2: &str) -> usize {
        hamming_distance(s1.as_bytes(), s2.as_bytes())
    }

    #[test]
    fn test_hamming_distance() {
        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";
        let actual_hamming = hamming_distance_of_strs(s1, s2);
        let expected_hamming = 37;
        assert_eq!(actual_hamming, expected_hamming);
    }
}
