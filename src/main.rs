use std::io;

fn main() {
    println!("input numbers of your pyramid steps.(1-255)");
    let steps = read_steps();

    println!("your pyramid steps is {}.", steps);
    // ここからピラミッド建立ゾーン
    let max = steps + 1;
    for i in 1..max {
        let brock = &String::from("■").repeat(i.into());
        let space = String::from(" ").repeat((max - i - 1).into());

        println!("{}", format!("{}{}{}", space, brock, space));
    }
}

// 関数に<'a>(生存期間パラメータ)をつけてあげて、&strも関数と同じだけ生きることを明示する
fn read_steps() -> u8 {
    loop {
        let mut steps = String::new();
        if io::stdin().read_line(&mut steps).is_err() {
            println!("Wwops, failed to read line. please re-enter.");
            continue;
        }

        let steps: Result<u8, std::num::ParseIntError> = steps.trim().parse();

        // steps.unwrap()だとResultの参照が死ぬのでas_ref()で参照型として取得した上で参照外し演算子*で参照先の値を取りにいく
        if steps.is_err() || *steps.as_ref().unwrap() == 0 {
            println!("Please re-enter number(1-255).");
            continue;
        }
        return steps.unwrap();
    }
}
