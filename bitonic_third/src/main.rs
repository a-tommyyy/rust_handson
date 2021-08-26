use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

#[allow(dead_code)]
enum SortOrder {
    Asc,
    Desc,
}

#[derive(Eq, Debug)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: String, age: u8) -> User {
        Self { name, age }
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.age.cmp(&other.age)
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}

fn main() {
    let user1 = User::new("a".to_string(), 1);
    let user2 = User::new("b".to_string(), 2);
    let user3 = User::new("c".to_string(), 3);
    let user4 = User::new("d".to_string(), 4);
    let mut list = vec![user1, user3, user4, user2];
    match sort(&mut list, Some(SortOrder::Asc)) {
        Ok(_) => println!("sort result: {:?}", list),
        Err(msg) => println!("sort failed. caused by {}", msg),
    }
}

fn sort<T: Ord>(list: &mut [T], option: Option<SortOrder>) -> Result<(), String> {
    if list.len().is_power_of_two() {
        match option {
            Some(direction) => match direction {
                SortOrder::Asc => do_sort(list, true),
                SortOrder::Desc => do_sort(list, false),
            },
            None => do_sort(list, true),
        }
        Ok(())
    } else {
        Err("The length is not a power of two.".to_string())
    }
}

fn do_sort<T: Ord>(list: &mut [T], asc: bool) {
    if list.len() > 1 {
        let mid_point = list.len() / 2;
        do_sort(&mut list[..mid_point], true);
        do_sort(&mut list[mid_point..], false);
        sub_sort(list, asc);
    }
}

fn sub_sort<T: Ord>(list: &mut [T], asc: bool) {
    if list.len() > 1 {
        compare_and_swap(list, asc);
        let mid_point = list.len() / 2;
        sub_sort(&mut list[..mid_point], asc);
        sub_sort(&mut list[mid_point..], asc);
    }
}

fn compare_and_swap<T: Ord>(list: &mut [T], asc: bool) {
    let mid_point = list.len() / 2;
    for i in 0..mid_point {
        if (list[i] > list[mid_point + i]) == asc {
            list.swap(i, mid_point + i)
        }
    }
}
