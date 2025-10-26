use std::fmt::format;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}



impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959
        }
    }
}


fn describe_state_quarter(coin:Coin) -> Option<String> {
    // if let Coin::Quarter(state) = coin {
    //     if state.existed_in(1900) {
    //         Some(format!("{state:?} is pretty old, for Amerika"))
    //     }else{
    //         Some(format!("{state:?} is relatively new."))
    //     }
    // }else {
    //     None
    // }

    //
    // let state = if let Coin::Quarter(state) = coin {
    //     state
    // }else {
    //     return None;
    // };
    //
    // if state.existed_in(1900) {
    //     Some(format!("{state:?} is pretty old, for America!"))
    // }else {
    //     format!("{state:?} is relatively new")
    // }

    let Coin::Quarter(state) = coin else {
        return None;
    };


    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        format!("{state:?} is relatively new")
    }
}


fn main() {
    let config_max = Some(3u8);
    let coin = Coin::Penny;

    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => ()
    }

    // if let supaya lebih ringkas tanpa boiler place "_"
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    };


    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!")
    }else{
        count += 1
    }


    println!("count {count}")


}
