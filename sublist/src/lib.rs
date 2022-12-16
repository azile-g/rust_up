use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// first try with if else conditionals 
pub fn first_sublist<T: PartialEq + Display>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;
    let first_str: String = _first_list.iter().map(|x: &T| x.to_string()).collect::<String>();
    let second_str: String = _second_list.iter().map(|x: &T| x.to_string()).collect::<String>();
    if first_str == second_str {
        return Equal
    }
    else if first_str.contains(&second_str) {
        return Superlist
    }
    else if second_str.contains(&first_str) {
        return Sublist
    }
    return Unequal
}

// second try with match 
pub fn sublist<T: Eq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (x, y) if x > y => if _first_list.windows(y).any(|z| z == _second_list) {Superlist} else {Unequal},
        (x, y) if x < y => if _second_list.windows(x).any(|z| z == _first_list) {Sublist} else {Unequal},
        (_, _) => if _first_list == _second_list {Equal} else {Unequal}
    }
}