use std::cmp::Ordering;

use itertools::Itertools;

use crate::{coprime_pairs::CoprimeGenerator, either3::Either3};

struct WordGridIndex {
  x: usize,
  y: usize,
}

pub struct WordGrid {
  grid: Vec<u8>,
  width: usize,
  height: usize,
}

impl WordGrid {
  pub fn new(grid: Vec<u8>, width: usize, height: usize) -> Self {
    debug_assert_eq!(width * height, grid.len());
    Self {
      grid,
      width,
      height,
    }
  }

  fn in_bounds(&self, pos: (i64, i64)) -> Option<WordGridIndex> {
    ((0..self.width as i64).contains(&pos.0) && (0..self.height as i64).contains(&pos.1)).then_some(
      WordGridIndex {
        x: pos.0 as usize,
        y: pos.1 as usize,
      },
    )
  }

  fn tile_at(&self, pos: WordGridIndex) -> u8 {
    self.grid[pos.x + pos.y * self.width]
  }

  fn all_grid_positions_within(
    xrange: (usize, usize),
    yrange: (usize, usize),
  ) -> impl Iterator<Item = (usize, usize)> {
    (xrange.0..xrange.1).flat_map(move |y| (yrange.0..yrange.1).map(move |x| (x, y)))
  }

  fn all_grid_positions(&self) -> impl Iterator<Item = (usize, usize)> {
    (0..self.height).flat_map(|y| (0..self.width).map(move |x| (x, y)))
  }

  fn all_grid_positions_for_delta(
    &self,
    (dx, dy): (i64, i64),
  ) -> impl Iterator<Item = (usize, usize)> {
    match dx.cmp(&0) {
      Ordering::Equal => Either3::A(std::iter::empty::<(usize, usize)>()),
      Ordering::Less => Either3::B(Self::all_grid_positions_within(
        ((self.width as i64 + dx) as usize, self.width),
        (0, self.height),
      )),
      Ordering::Greater => Either3::C(Self::all_grid_positions_within(
        (0, dx as usize),
        (0, self.height),
      )),
    }
    .chain(match dy.cmp(&0) {
      Ordering::Equal => Either3::A(std::iter::empty::<(usize, usize)>()),
      Ordering::Less => Either3::B(Self::all_grid_positions_within(
        (0, self.width),
        ((self.height as i64 + dy) as usize, self.height),
      )),
      Ordering::Greater => Either3::C(Self::all_grid_positions_within(
        (0, self.width),
        (0, dy as usize),
      )),
    })
  }

  fn all_candidate_lines(&self) -> impl Iterator<Item = Vec<u8>> {
    CoprimeGenerator::new((self.width.max(self.height) - 1) as u64)
      .filter(|&(dy, dx)| dx < self.width as u64 && dy < self.height as u64)
      .flat_map(move |(dy, dx)| {
        let dx = dx as i64;
        let dy = dy as i64;

        println!(
          "{},{}: {},{} {},{} {},{} {},{}",
          dx,
          dy,
          dx,
          dy,
          dx - dy,
          dx,
          -dy,
          dx,
          -dx,
          dx - dy
        );

        [(dx, dy), (dx - dy, dx), (-dy, dx), (-dx, dx - dy)]
          .into_iter()
          .flat_map(move |(dx, dy)| {
            self
              .all_grid_positions_for_delta((dx, dy))
              .map(move |(x, y)| {
                std::iter::successors(Some(WordGridIndex { x, y }), |pos| {
                  let x = pos.x as i64 + dx;
                  let y = pos.y as i64 + dy;
                  self.in_bounds((x, y))
                })
                .inspect(|pos| println!("  {x},{y}: {},{}", pos.x, pos.y))
                .map(|pos| self.tile_at(pos))
                .collect_vec()
              })
              .filter(|word| word.len() > 1)
              .inspect(|word| println!("    {}", str::from_utf8(word).unwrap()))
          })
      })
      .flat_map(|word| {
        let mut reversed = word.clone();
        reversed.reverse();
        [word, reversed]
      })
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use googletest::prelude::*;
  use itertools::Itertools;

  use crate::word_grid::WordGrid;

  fn all_grid_positions_for_delta_slow(
    width: usize,
    height: usize,
    (dx, dy): (i64, i64),
  ) -> impl Iterator<Item = (usize, usize)> {
    (0..width)
      .flat_map(move |x| (0..height).map(move |y| (x, y)))
      .filter(move |&(x, y)| {
        !(0..width as i64).contains(&(x as i64 - dx))
          || !(0..height as i64).contains(&(y as i64 - dy))
      })
  }

  #[gtest]
  fn test_all_grid_positions_for_delta() {
    let mut all_positions = WordGrid::new(vec![b'a'; 6], 3, 2)
      .all_grid_positions_for_delta((1, 2))
      .collect_vec();
    all_positions.sort_by_key(|(x, y)| x + y * 3);
    expect_eq!(
      all_positions,
      all_grid_positions_for_delta_slow(3, 2, (1, 2)).collect_vec()
    );
  }

  #[gtest]
  fn test_2x2() {
    // a b
    // c d
    let tiles = vec![b'a', b'b', b'c', b'd'];
    let grid = WordGrid::new(tiles, 2, 2);

    expect_that!(
      grid.all_candidate_lines().collect_vec(),
      unordered_elements_are![
        &vec![b'a', b'b'],
        &vec![b'b', b'a'],
        &vec![b'a', b'c'],
        &vec![b'c', b'a'],
        &vec![b'a', b'd'],
        &vec![b'd', b'a'],
        &vec![b'b', b'c'],
        &vec![b'c', b'b'],
        &vec![b'b', b'd'],
        &vec![b'd', b'b'],
        &vec![b'c', b'd'],
        &vec![b'd', b'c'],
      ]
    );
  }

  #[gtest]
  fn test_3x2() {
    // a b c
    // d e f
    let tiles = vec![b'a', b'b', b'c', b'd', b'e', b'f'];
    let grid = WordGrid::new(tiles, 3, 2);

    expect_that!(
      grid.all_candidate_lines().collect_vec(),
      unordered_elements_are![
        &vec![b'a', b'b', b'c'],
        &vec![b'c', b'b', b'a'],
        &vec![b'a', b'e'],
        &vec![b'e', b'a'],
        &vec![b'a', b'd'],
        &vec![b'd', b'a'],
        &vec![b'a', b'f'],
        &vec![b'f', b'a'],
        &vec![b'b', b'd'],
        &vec![b'd', b'b'],
        &vec![b'b', b'e'],
        &vec![b'e', b'b'],
        &vec![b'b', b'f'],
        &vec![b'f', b'b'],
        &vec![b'c', b'e'],
        &vec![b'e', b'c'],
        &vec![b'c', b'f'],
        &vec![b'f', b'c'],
        &vec![b'c', b'd'],
        &vec![b'd', b'c'],
        &vec![b'd', b'e', b'f'],
        &vec![b'f', b'e', b'd'],
      ]
    );
  }
}
