const NUM_CHARS: usize = 26;

trait SplitFirstChar {
  fn split_first_letter(&self) -> Option<(u8, &[u8])>;
}

impl SplitFirstChar for &[u8] {
  fn split_first_letter(&self) -> Option<(u8, &[u8])> {
    Some((*self.first()?, &self[1..]))
  }
}

struct PrefixTree {
  prefixes: [Option<Box<PrefixTree>>; NUM_CHARS],
  terminal: bool,
}

impl PrefixTree {
  fn new() -> Self {
    Self {
      prefixes: [(); NUM_CHARS].map(|_| None),
      terminal: false,
    }
  }

  fn char_index(c: u8) -> usize {
    debug_assert!((b'a'..=b'z').contains(&c));
    return c as usize - (b'a' as usize);
  }

  fn mut_subtree(&mut self, c: u8) -> &mut PrefixTree {
    self.prefixes[Self::char_index(c)]
      .get_or_insert_with(|| Box::new(PrefixTree::new()))
      .as_mut()
  }

  fn insert_word(&mut self, word: &[u8]) {
    if let Some((letter, remainder)) = word.split_first_letter() {
      self.mut_subtree(letter).insert_word(remainder);
    } else {
      self.terminal = true;
    }
  }

  fn find_all_words<'a>(&self, stream: &'a [u8]) -> impl Iterator<Item = &'a [u8]> {
    let mut tree = Some(self);
    let mut index = 0;
    std::iter::from_fn(move || {
      tree.map(|cur_tree| {
        tree = cur_tree.prefixes[Self::char_index(stream[index])].as_deref();
        let result = cur_tree.terminal.then_some(&stream[..index]);
        index += 1;
        result
      })
    })
    .flatten()
  }
}

struct Dictionary {
  prefix_tree: PrefixTree,
}

impl Dictionary {
  pub fn new<'a>(word_list: impl IntoIterator<Item = &'a str>) -> Self {
    let prefix_tree = word_list
      .into_iter()
      .fold(PrefixTree::new(), |mut prefix_tree, word| {
        prefix_tree.insert_word(word.as_bytes());
        prefix_tree
      });
    Self { prefix_tree }
  }

  fn find_all_words<'a>(&self, stream: &'a [u8]) -> impl Iterator<Item = &'a [u8]> {
    self.prefix_tree.find_all_words(stream)
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use itertools::Itertools;

  use crate::dictionary::Dictionary;

  #[gtest]
  fn all_matches() {
    let dictionary = Dictionary::new(["ab", "abcde"]);

    expect_that!(
      dictionary
        .find_all_words("abcdefgh".as_bytes())
        .collect_vec(),
      unordered_elements_are![&"ab".as_bytes(), &"abcde".as_bytes()]
    );
  }

  #[gtest]
  fn no_matches() {
    let dictionary = Dictionary::new(["ab", "cde"]);

    expect_that!(
      dictionary.find_all_words("bcdeab".as_bytes()).collect_vec(),
      is_empty()
    );
  }

  #[gtest]
  fn some_matches() {
    let dictionary = Dictionary::new(["ab", "cde"]);

    expect_that!(
      dictionary.find_all_words("cdeab".as_bytes()).collect_vec(),
      elements_are![&"cde".as_bytes()]
    );
  }

  #[gtest]
  fn complex_prefixes() {
    let dictionary = Dictionary::new(["ab", "abcde", "abdce", "acde", "abcdf"]);

    expect_that!(
      dictionary.find_all_words("abcdef".as_bytes()).collect_vec(),
      unordered_elements_are![&"ab".as_bytes(), &"abcde".as_bytes()]
    );
  }
}
