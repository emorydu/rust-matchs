#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn main() {
    let result = value_in_cents(Coin::Penny);
    println!("result: {}", result);

    let result = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("result: {}", result);

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {six:?}");
    let none = plus_one(None);
    println!("none: {none:?}");

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }


    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }


    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }


    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // let mut count = 0;
    // math coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}"),
    //     _ => count += 1,
    // }


    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {state:?}");
    // } else {
    //     count += 1;
    // }
}
