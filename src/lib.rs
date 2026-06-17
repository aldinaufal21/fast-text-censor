#![deny(clippy::all)]

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct TextCensor {
  ac: AhoCorasick,
  patterns: Vec<String>,
}

#[napi]
impl TextCensor {
  #[napi(constructor)]
  pub fn new(words: Vec<String>) -> Result<Self> {
    let mut normalized_patterns = Vec::with_capacity(words.len());
    for word in &words {
      normalized_patterns.push(normalize_text(word).0);
    }

    let ac = AhoCorasickBuilder::new()
      .ascii_case_insensitive(true)
      .match_kind(MatchKind::LeftmostLongest)
      .build(&normalized_patterns)
      .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

    Ok(Self {
      ac,
      patterns: words,
    })
  }

  #[napi]
  pub fn clean(&self, text: String) -> String {
    let (normalized_text, byte_map) = normalize_text(&text);
    
    let mut result = String::with_capacity(text.len());
    let mut last_match_orig = 0;
    
    for mat in self.ac.find_iter(&normalized_text) {
      let orig_start = byte_map[mat.start()];
      let orig_end = byte_map[mat.end()];
      
      if orig_start < last_match_orig {
        continue;
      }

      result.push_str(&text[last_match_orig..orig_start]);
      
      let chars_count = text[orig_start..orig_end].chars().count();
      result.push_str(&"*".repeat(chars_count));
      
      last_match_orig = orig_end;
    }
    
    if last_match_orig < text.len() {
      result.push_str(&text[last_match_orig..]);
    }
    
    result
  }

  #[napi]
  pub fn add_words(&mut self, words: Vec<String>) -> Result<()> {
    self.patterns.extend(words);
    
    let mut normalized_patterns = Vec::with_capacity(self.patterns.len());
    for word in &self.patterns {
      normalized_patterns.push(normalize_text(word).0);
    }
    
    let ac = AhoCorasickBuilder::new()
      .ascii_case_insensitive(true)
      .match_kind(MatchKind::LeftmostLongest)
      .build(&normalized_patterns)
      .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
      
    self.ac = ac;
    Ok(())
  }
}

fn normalize_text(text: &str) -> (String, Vec<usize>) {
  let mut normalized = String::with_capacity(text.len());
  let mut byte_map = Vec::with_capacity(text.len() + 1);
  
  let mut prev_char = '\0';
  
  for (orig_idx, c) in text.char_indices() {
    let mut norm_c = c.to_ascii_lowercase();
    
    // Leetspeak conversion
    norm_c = match norm_c {
      '@' | '4' => 'a',
      '0' => 'o',
      '1' | '!' => 'i',
      '3' => 'e',
      '5' | '$' => 's',
      '7' => 't',
      '8' => 'b',
      _ => norm_c,
    };
    
    // Skip punctuation
    if norm_c.is_ascii_punctuation() && norm_c != ' ' {
      continue;
    }
    
    // Remove repeated characters
    if norm_c == prev_char && norm_c != ' ' {
      continue;
    }
    
    prev_char = norm_c;
    
    let mut buf = [0; 4];
    let encoded = norm_c.encode_utf8(&mut buf);
    for _ in 0..encoded.len() {
      byte_map.push(orig_idx);
    }
    normalized.push_str(encoded);
  }
  
  byte_map.push(text.len());
  
  (normalized, byte_map)
}
