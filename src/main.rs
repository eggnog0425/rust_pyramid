use std::io;

fn main() {
    println!("input numbers of your pyramid steps.(1-255)");
    let steps = read_steps();

    println!("your pyramid steps is {}.", steps);
    build_pyramid(steps);
}

fn read_steps() -> u8 {
    loop {
        let mut steps = String::new();
        let input = io::stdin().read_line(&mut steps);
        if valid_input(&input, &steps) {
            return steps.trim().parse().unwrap();
        }
        continue;
    }
}

fn valid_input(input: &io::Result<usize>, steps: &String) -> bool {
    if input.is_err() {
        println!("Wwops, failed to read line. please re-enter.");
        return false;
    }

    let steps: Result<u8, std::num::ParseIntError> = steps.trim().parse();

    if steps.is_err() || *steps.as_ref().unwrap() == 0 {
        println!("Please re-enter number(1-255).");
        return false;
    }
    true
}

fn build_pyramid(steps: u8) {
    let steps: u16 = From::from(steps);
    let max = steps + 1;

    for i in 1..max {
        let brock = &String::from("■").repeat(i.into());
        let space = String::from(" ").repeat((max - i - 1).into());

        println!("{}", format!("{}{}{}", space, brock, space));
    }
}

#[cfg(test)]
fn test_read_steps() {
    // 空文字
    // 半角スペース
    // 2byte文字列
    // -1
    // 0
    // 1
    // 255
    // 256
}
