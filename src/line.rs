use alloc::string::String;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

use super::{Input, Lines, State};

#[derive(Eq)]
pub struct Line {
  offset: usize,
  line: String,
}

impl Hash for Line {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.line.hash(state);
  }
}

impl PartialEq for Line {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.line.eq(&other.line)
  }
}

impl PartialOrd for Line {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.line.partial_cmp(&other.line)
  }
}

impl Ord for Line {
  fn cmp(&self, other: &Self) -> Ordering {
    self.line.cmp(&other.line)
  }
}

impl<'a> From<&'a str> for Line {
  #[inline]
  fn from(string: &'a str) -> Self {
    Line {
      offset: 0,
      line: string.into(),
    }
  }
}

impl From<(usize, String)> for Line {
  #[inline]
  fn from((offset, line): (usize, String)) -> Self {
    Line {
      offset: offset,
      line: line,
    }
  }
}

impl Deref for Line {
  type Target = String;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.line
  }
}

impl DerefMut for Line {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.line
  }
}

impl Input for Line {
  #[inline]
  fn peek(&mut self, state: &State, offset: usize) -> Option<char> {
    self
      .line
      .chars()
      .nth((state.index() - self.offset) + offset)
  }
  #[inline]
  fn lines<'a>(&'a mut self, state: &'a mut State) -> Lines<'a> {
    Lines::new(self, state)
  }
}

impl fmt::Debug for Line {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", &self.line)
  }
}

impl fmt::Display for Line {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", &self.line)
  }
}
