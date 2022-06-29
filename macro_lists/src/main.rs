struct List {
    item: Item,
}

enum Item {
    Thing(i32),
    List(Box<List>),
}

macro_rules! list {
    ($x:expr) => {
        List {
            item: Item::Thing($x),
        }
    }; //($list:tt) =>  {
       //}
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn it_works() {
    let list = list!(1);

    match list.item {
        Item::Thing(x) => assert_eq!(1, x),
        _ => panic!(),
    };
}
