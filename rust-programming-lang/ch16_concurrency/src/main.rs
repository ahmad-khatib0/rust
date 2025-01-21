mod basics;
mod message_passing;
mod shared_state;

fn main() {
    println!("Hello, world!");

    basics::basics();

    message_passing::message_passing();

    shared_state::shared_state();
}
