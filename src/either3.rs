pub enum Either3<A, B, C> {
  A(A),
  B(B),
  C(C),
}

impl<A, B, C, I> Iterator for Either3<A, B, C>
where
  A: Iterator<Item = I>,
  B: Iterator<Item = I>,
  C: Iterator<Item = I>,
{
  type Item = I;

  fn next(&mut self) -> Option<I> {
    match self {
      Self::A(a) => a.next(),
      Self::B(b) => b.next(),
      Self::C(c) => c.next(),
    }
  }
}
