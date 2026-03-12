struct CoprimeGenerator {
  n: u64,
  ab: (u64, u64),
  cd: (u64, u64),
}

impl CoprimeGenerator {
  pub fn new(n: u64) -> Self {
    Self {
      n,
      ab: (0, 1),
      cd: (1, n),
    }
  }
}

impl Iterator for CoprimeGenerator {
  type Item = (u64, u64);

  fn next(&mut self) -> Option<Self::Item> {
    let (a, b) = self.ab;
    let (c, d) = self.cd;

    if a > self.n {
      return None;
    }

    let k = (self.n + b) / d;
    let e = k * c - a;
    let f = k * d - b;

    self.ab = (c, d);
    self.cd = (e, f);

    Some((a, b))
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use itertools::Itertools;

  use crate::coprime_pairs::CoprimeGenerator;

  #[gtest]
  fn until_2() {
    expect_that!(
      CoprimeGenerator::new(2).collect_vec(),
      unordered_elements_are![&(0, 1), &(1, 1), &(1, 2)],
    );
  }

  #[gtest]
  fn until_3() {
    expect_that!(
      CoprimeGenerator::new(3).collect_vec(),
      unordered_elements_are![&(0, 1), &(1, 1), &(1, 2), &(1, 3), &(2, 3)],
    );
  }

  #[gtest]
  fn until_4() {
    expect_that!(
      CoprimeGenerator::new(4).collect_vec(),
      unordered_elements_are![
        &(0, 1),
        &(1, 1),
        &(1, 2),
        &(1, 3),
        &(2, 3),
        &(1, 4),
        &(3, 4),
      ],
    );
  }

  #[gtest]
  fn until_5() {
    expect_that!(
      CoprimeGenerator::new(5).collect_vec(),
      unordered_elements_are![
        &(0, 1),
        &(1, 1),
        &(1, 2),
        &(1, 3),
        &(2, 3),
        &(1, 4),
        &(3, 4),
        &(1, 5),
        &(2, 5),
        &(3, 5),
        &(4, 5),
      ],
    );
  }

  #[gtest]
  fn until_6() {
    expect_that!(
      CoprimeGenerator::new(6).collect_vec(),
      unordered_elements_are![
        &(0, 1),
        &(1, 1),
        &(1, 2),
        &(1, 3),
        &(2, 3),
        &(1, 4),
        &(3, 4),
        &(1, 5),
        &(2, 5),
        &(3, 5),
        &(4, 5),
        &(1, 6),
        &(5, 6),
      ],
    );
  }
}
