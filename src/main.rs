mod misc;

fn main() {
    let time = misc::Length::from_int(369);

    println!("Hello, world! {}", time.pretty_print());
}
