mod example;
use example::run as example_run;

fn main() {
    let connected = example_run(vec![(1, 2), (2, 3)]);
    println!("Connected: {connected:?}");
}
