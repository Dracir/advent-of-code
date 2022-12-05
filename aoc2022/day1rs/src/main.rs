fn main() {
    println!("Hello, world!");

    let input = std::fs::read_to_string("../examples/day1/example0.txt")
        .expect("Should have been able to read the file");
    let elves = input.replace("\n", "");
    let elves = elves.split("\n\n");

    for line in elves {
        println!("{} - ", line);
    }
}
