fn main() {
    println!("Hello, world!");
}

#[test]
fn part1_exemple1_test() {
    assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
}

#[test]
fn part1_exemple2_test() {
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
}

#[test]
fn part1_exemple3_test() {
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
}

#[test]
fn part1_exemple4_test() {
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
}
#[test]
fn part1_exemple5_test() {
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
}

fn part1(input: &str, markerSize: i32) -> i32 {
    // create a vector of characters
    let mut marker: Vec<char> = Vec::new();
    // fill the vector with markerSize - 1 first characters
    for c in input.chars().take(markerSize as usize - 1) {
        marker.push(c);
    }
    // for each index of character starting at markerSize - 1 to the end
	for (i, c) in input.chars().enumerate().skip(markerSize as usize - 1) {
		// add the character to the vector
		marker.push(c);
		// if marker contains only dictinct characters return index + 1
		if marker.iter().all(|&x| marker.iter().filter(|&y| x == *y).count() == 1) {
			return (i + 1) as i32;
		}
		// remove the first character of the vector
		marker.remove(0);
	}

    return 0;
}

//Fonction that returns an integer
