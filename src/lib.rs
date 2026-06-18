#![deny(clippy::all)]

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use napi::Task;
#[napi(object)]
pub struct CensorOptions {
  pub custom_char: Option<String>,
  pub keep_first_last: Option<bool>,
  pub match_whole_word: Option<bool>,
  pub ignore_list: Option<Vec<String>>,
}

#[napi(object)]
pub struct MatchResult {
  pub word: String,
  pub match_text: String,
  pub start: u32,
  pub end: u32,
}

pub struct CleanTask {
  ac: Arc<AhoCorasick>,
  patterns: Arc<Vec<String>>,
  text: String,
  options: Option<CensorOptions>,
}

#[napi]
impl Task for CleanTask {
  type Output = String;
  type JsValue = napi::JsString;

  fn compute(&mut self) -> Result<Self::Output> {
    let (normalized_text, byte_map) = normalize_text(&self.text);
    
    let mut result = String::with_capacity(self.text.len());
    let mut last_match_orig = 0;
    
    let repl_char = self.options.as_ref().and_then(|o| o.custom_char.as_deref()).unwrap_or("*");
    let keep_first_last = self.options.as_ref().and_then(|o| o.keep_first_last).unwrap_or(false);
    let match_whole_word = self.options.as_ref().and_then(|o| o.match_whole_word).unwrap_or(false);
    let ignore_list = self.options.as_ref().and_then(|o| o.ignore_list.as_deref()).unwrap_or(&[]);

    for mat in self.ac.find_iter(&normalized_text) {
      let pattern_id = mat.pattern().as_usize();
      let matched_pattern = &self.patterns[pattern_id];
      if ignore_list.iter().any(|w| w.eq_ignore_ascii_case(matched_pattern)) {
          continue;
      }
      
      let orig_start = byte_map[mat.start()];
      let orig_end = byte_map[mat.end()];
      
      if match_whole_word {
          let is_start_boundary = orig_start == 0 || self.text[..orig_start].chars().last().map_or(true, |c| !c.is_alphanumeric());
          let is_end_boundary = orig_end == self.text.len() || self.text[orig_end..].chars().next().map_or(true, |c| !c.is_alphanumeric());
          
          if !is_start_boundary || !is_end_boundary {
              continue;
          }
      }
      
      if orig_start < last_match_orig {
        continue;
      }

      result.push_str(&self.text[last_match_orig..orig_start]);
      
      let matched_str = &self.text[orig_start..orig_end];
      let chars_count = matched_str.chars().count();
      
      if keep_first_last && chars_count > 2 {
          let first_char = matched_str.chars().next().unwrap();
          let last_char = matched_str.chars().last().unwrap();
          result.push(first_char);
          result.push_str(&repl_char.repeat(chars_count - 2));
          result.push(last_char);
      } else {
          result.push_str(&repl_char.repeat(chars_count));
      }
      
      last_match_orig = orig_end;
    }
    
    if last_match_orig < self.text.len() {
      result.push_str(&self.text[last_match_orig..]);
    }
    
    Ok(result)
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_string(&output)
  }
}

#[napi]
pub struct TextCensor {
  ac: Arc<AhoCorasick>,
  patterns: Arc<Vec<String>>,
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
      ac: Arc::new(ac),
      patterns: Arc::new(words),
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
  pub fn clean_custom(&self, text: String, options: CensorOptions) -> String {
    let (normalized_text, byte_map) = normalize_text(&text);
    
    let mut result = String::with_capacity(text.len());
    let mut last_match_orig = 0;
    
    let repl_char = options.custom_char.as_deref().unwrap_or("*");
    let keep_first_last = options.keep_first_last.unwrap_or(false);
    let match_whole_word = options.match_whole_word.unwrap_or(false);
    let ignore_list = options.ignore_list.as_deref().unwrap_or(&[]);

    for mat in self.ac.find_iter(&normalized_text) {
      let pattern_id = mat.pattern().as_usize();
      let matched_pattern = &self.patterns[pattern_id];
      if ignore_list.iter().any(|w| w.eq_ignore_ascii_case(matched_pattern)) {
          continue;
      }
      
      let orig_start = byte_map[mat.start()];
      let orig_end = byte_map[mat.end()];
      
      if match_whole_word {
          let is_start_boundary = orig_start == 0 || text[..orig_start].chars().last().map_or(true, |c| !c.is_alphanumeric());
          let is_end_boundary = orig_end == text.len() || text[orig_end..].chars().next().map_or(true, |c| !c.is_alphanumeric());
          
          if !is_start_boundary || !is_end_boundary {
              continue;
          }
      }
      
      if orig_start < last_match_orig {
        continue;
      }

      result.push_str(&text[last_match_orig..orig_start]);
      
      let matched_str = &text[orig_start..orig_end];
      let chars_count = matched_str.chars().count();
      
      if keep_first_last && chars_count > 2 {
          let first_char = matched_str.chars().next().unwrap();
          let last_char = matched_str.chars().last().unwrap();
          result.push(first_char);
          result.push_str(&repl_char.repeat(chars_count - 2));
          result.push(last_char);
      } else {
          result.push_str(&repl_char.repeat(chars_count));
      }
      
      last_match_orig = orig_end;
    }
    
    if last_match_orig < text.len() {
      result.push_str(&text[last_match_orig..]);
    }
    
    result
  }

  #[napi]
  pub fn get_matches(&self, text: String, options: Option<CensorOptions>) -> Vec<MatchResult> {
    let (normalized_text, byte_map) = normalize_text(&text);
    
    let mut matches = Vec::new();
    let match_whole_word = options.as_ref().and_then(|o| o.match_whole_word).unwrap_or(false);
    let ignore_list = options.as_ref().and_then(|o| o.ignore_list.as_deref()).unwrap_or(&[]);
    
    let mut utf16_indices = Vec::with_capacity(text.len() + 1);
    let mut current_utf16 = 0;
    for c in text.chars() {
      utf16_indices.push(current_utf16);
      for _ in 1..c.len_utf8() {
        utf16_indices.push(current_utf16);
      }
      current_utf16 += c.len_utf16() as u32;
    }
    utf16_indices.push(current_utf16);

    let mut last_match_orig = 0;

    for mat in self.ac.find_iter(&normalized_text) {
      let pattern_id = mat.pattern().as_usize();
      let matched_pattern = &self.patterns[pattern_id];
      if ignore_list.iter().any(|w| w.eq_ignore_ascii_case(matched_pattern)) {
          continue;
      }
      
      let orig_start = byte_map[mat.start()];
      let orig_end = byte_map[mat.end()];
      
      if match_whole_word {
          let is_start_boundary = orig_start == 0 || text[..orig_start].chars().last().map_or(true, |c| !c.is_alphanumeric());
          let is_end_boundary = orig_end == text.len() || text[orig_end..].chars().next().map_or(true, |c| !c.is_alphanumeric());
          
          if !is_start_boundary || !is_end_boundary {
              continue;
          }
      }
      
      if orig_start < last_match_orig {
        continue;
      }

      let start_utf16 = utf16_indices[orig_start];
      let end_utf16 = utf16_indices[orig_end];

      matches.push(MatchResult {
        word: matched_pattern.clone(),
        match_text: text[orig_start..orig_end].to_string(),
        start: start_utf16,
        end: end_utf16,
      });
      
      last_match_orig = orig_end;
    }
    
    matches
  }

  #[napi]
  pub fn clean_async(&self, text: String) -> AsyncTask<CleanTask> {
    AsyncTask::new(CleanTask {
      ac: Arc::clone(&self.ac),
      patterns: Arc::clone(&self.patterns),
      text,
      options: None,
    })
  }

  #[napi]
  pub fn clean_custom_async(&self, text: String, options: CensorOptions) -> AsyncTask<CleanTask> {
    AsyncTask::new(CleanTask {
      ac: Arc::clone(&self.ac),
      patterns: Arc::clone(&self.patterns),
      text,
      options: Some(options),
    })
  }

  #[napi]
  pub fn add_words(&mut self, words: Vec<String>) -> Result<()> {
    let mut new_patterns = self.patterns.as_ref().clone();
    new_patterns.extend(words);
    
    let mut normalized_patterns = Vec::with_capacity(new_patterns.len());
    for word in &new_patterns {
      normalized_patterns.push(normalize_text(word).0);
    }
    
    let ac = AhoCorasickBuilder::new()
      .ascii_case_insensitive(true)
      .match_kind(MatchKind::LeftmostLongest)
      .build(&normalized_patterns)
      .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
      
    self.ac = Arc::new(ac);
    self.patterns = Arc::new(new_patterns);
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
      '9' => 'g',
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
