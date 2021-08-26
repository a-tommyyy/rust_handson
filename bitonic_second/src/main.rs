use std::cmp::Ord;

#[allow(dead_code)]
enum SortOrder {
    Asc,
    Desc,
}

fn main() {
    let mut list: Vec<char> = vec!['a', 'c', 'e', 'g', 'h', 'f', 'd', 'b'];
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
