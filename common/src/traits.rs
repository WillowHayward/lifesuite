use crate::r#mod::Change;

pub trait Diff {
    fn diff<T>(_old: Self, _new: Self) -> Vec<Change<T>>;
}
