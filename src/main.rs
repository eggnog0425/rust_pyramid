use std::io;

fn main() {
    println!("input numbers of your pyramid steps.(1-255)");
    let steps = read_steps();

    println!("your pyramid steps is {}.", steps);
    build_pyramid(steps);
}

fn read_steps() -> u16 {
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

    let steps: Result<u16, std::num::ParseIntError> = steps.trim().parse();

    match steps {
        Ok(_v) => {
            let i = steps.unwrap();
            if 0 < i && i < 256 {
                return true;
            }
            println!("Please re-enter number(1-255).");
            return false;
        }
        Err(_e) => {
            println!("Please re-enter number(1-255).");
            return false;
        }
    }
}

fn build_pyramid(steps: u16) {
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
