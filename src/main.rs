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
        let brock = &String::from("*").repeat(i.into());
        let space = String::from(" ").repeat((max - i - 1).into());

        println!("{}", format!("{}{}{}", space, brock, space));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_input() {
        use super::*;
        use std::io::{Error as ioError, ErrorKind as ioErrorKind};
        // ioerror
        let result_error = &Err(ioError::new(ioErrorKind::Other, "error"));
        assert_eq!(false, valid_input(result_error, &String::from("")));

        // 空文字
        assert_eq!(false, valid_input(&Ok(2), &String::from(""))); // 改行コードが入るので2バイト(多分OS依存するから正しく書くなら場合分けすべきだけどテストコードなので気にしない)

        // 半角スペース
        let result_1byte = &Ok(3);
        assert_eq!(false, valid_input(result_1byte, &String::from(" ")));

        // -1
        assert_eq!(false, valid_input(&Ok(4), &String::from("-1")));

        // 0
        assert_eq!(false, valid_input(result_1byte, &String::from("0")));

        // 1
        assert_eq!(true, valid_input(result_1byte, &String::from("1")));

        // 255
        let result_3byte = &Ok(5);
        assert_eq!(true, valid_input(result_3byte, &String::from("255")));

        // 256
        assert_eq!(false, valid_input(result_3byte, &String::from("256")));
    }

    #[test]
    // valid_inputで1-255の数値であることを保証しているので、最小値、最小値+1、最大値のみテスト
    fn test_build_pyramid() {
        use super::*;
        build_pyramid(1);
        build_pyramid(2);
        build_pyramid(255);
    }
}
