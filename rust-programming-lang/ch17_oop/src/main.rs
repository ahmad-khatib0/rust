use crate::blog::Post;
mod blog_as_types;

mod blog;
mod gui;

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl gui::Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    println!("Hello, world!");

    let mut ac = AveragedCollection {
        list: vec![],
        average: 0.0,
    };

    ac.add(44);
    ac.add(55);
    ac.add(24);

    let average = ac.average();
    println!("{average}");

    let screen = gui::Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(gui::Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // The state pattern is an object-oriented design pattern. The crux of the pattern is that
    // we define a set of states a value can have internally. The states are represented by a
    // set of state objects, and the value’s behavior changes based on its state.

    let mut post1 = blog_as_types::Post::new();
    post1.add_text("I ate a salad for lunch today");
    let post = post1.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
