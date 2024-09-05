#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq> (first_list: &[T], second_list: &[T]) -> Comparison {
    let superlist = second_list.is_empty()
}
