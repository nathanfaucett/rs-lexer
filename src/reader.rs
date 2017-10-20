use super::{Input, State};


pub trait Reader<T>: Sync + Send {

    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(&self, &Input, &State, &mut State) -> Option<T>;
}
