enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn add(num: i32, num2: Option<i32>) => {
    match num2 {
        Some(i: i32) => num + i,
        None => num,
    }
}

fn main() {
    let dice_roll: i32 = 0;
    match dice_roll {
        3 => println!("you got fancy hat"),
        6 => println!("You got hat removed"),
        other => println!("Move {dice_roll} steps"), // If not other you can also use `_` its same
                                                     // as other
        _ => reroll(),
    };
}
