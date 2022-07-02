#[derive(Debug, PartialEq, Eq)]
pub enum Item<T> {
    Single(T),
    Many(Vec<Item<T>>),
}

/// TODO: This macro does not output the same type, which should always
///  be Item<T>, but now we get Item<Item<T>>, etc.
///  ```rust
///  // Usage
///  use macro_lists::{Item, list};
///  let list = macro_lists::list!((123 32) (23));
///  ```
#[macro_export]
macro_rules! list {
    ($x:literal) => {
        Item::Single($x)
    };

    ($( $x:literal )* ) => {
        Item::Many(vec![$( Item::Single($x) ),*])
    };

    ( ( $($inner_list:literal)*)) => {
        Item::Single(list!($($inner_list) *))
    };

    ($( $tokentree:tt )*) => {
        Item::Many(vec![$( list!($tokentree) ),*])
    };
}

#[cfg(test)]
mod tests {

    use std::ops::Index;

    use super::{
        Item::{Many, Single},
        *,
    };

    #[test]
    fn it_works() {
        let list = list!(1);

        match list {
            Single(x) => assert_eq!(x, 1),
            Many(_) => panic!(),
        };
    }

    #[test]
    fn mulitple_things() {
        let list = list!(1 2 3);

        match list {
            Item::Single(_) => panic!("Got a single thing"),
            Item::Many(v) => assert_eq!(vec![Single(1), Single(2), Single(3)], v),
        }
    }

    #[test]
    fn things_in_things() {
        let list_of_lists = list!((1 2) (1));
        let v = match list_of_lists {
            Single(_) => panic!(),
            Many(v) => v,
        };

        let first = v.first().unwrap();
        let second = v.index(1);

        match first {
            Single(v) => assert_eq!(*v, list!(1 2)),
            _ => panic!(),
        }
        match second {
            Single(v) => assert_eq!(*v, list!(1)),
            _ => panic!(),
        }
    }
}
