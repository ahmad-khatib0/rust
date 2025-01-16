#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

pub fn fn_traits() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // The reason sort_by_key is defined to take an FnMut closure is that it
    // calls the closure multiple times: once for each item in the slice.
    // the closure |r| r.width doesn’t capture, mutate, or move anything out
    // from its environment, so it meets the trait bound requirements.
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // In contrast, Listing 13-8 shows an example of a closure that
    // implements just the FnOnce trait, because it moves a value out of the environment.

    let mut sort_operations: Vec<String> = vec![];
    let value = String::from("by key called");
    list.sort_by_key(|r| {
        // cannot move out of `value`, a captured variable in an `FnMut` closure
        // move occurs because `value` has type `String`, which does not implement the `Copy` trait
        // sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
    // This is a contrived, convoluted way (that doesn’t work) to try and count the number of times
    // sort_by_key gets called when sorting list. This code attempts to do this counting by pushing
    // value — a String from the closure’s environment—into the sort_operations vector. The closure
    // captures value and then moves value out of the closure by transferring ownership of value to
    // the sort_operations vector. This closure can be called once; trying to call it a second time
    // wouldn’t work because value would no longer be in the environment to be pushed into
    // sort_operations again! Therefore, this closure only implements FnOnce.
    //

    // The error points to the line in the closure body that moves value out of the environment.
    // To fix this, we need to change the closure body so that it doesn’t move values out of the
    // environment. Keeping a counter in the environment and incrementing its value in the closure
    // body is a more straightforward way to count the number of times sort_by_key is called.
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        // Using an FnMut closure with sort_by_key is allowed
        num_sort_operations += 1;
        r.width
    });

    // Calling the iterator adapter map to create a new iterator
    let v1: Vec<i32> = vec![1, 2, 3];
    let n = v1.iter().map(|x| x + 1);
    dbg!(n);
    // iterator adapters are lazy, and we need to consume the iterator here.

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Using Closures That Capture Their Environment
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
