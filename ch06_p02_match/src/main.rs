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

fn main() {

    let dime    = Coin::Dime;
    let penny   = Coin::Penny;
    let nickel  = Coin::Nickel;
    let quarter = Coin::Quarter(UsState::Alabama);
    let quarter2 = Coin::Quarter(UsState::Alaska);

    let mut value = value_in_cents(dime);
    println!("Dime in cents: {value}");

    value = value_in_cents(penny);
    println!("Penny in cents: {value}");

    value = value_in_cents(nickel);
    println!("Nickel in cents: {value}");

    value = value_in_cents(quarter);
    println!("Quarter in cents: {value}");

    value = value_in_cents(quarter2);
    println!("Quarter in cents: {value}");
 

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:#?}", six);
    println!("{:#?}", none);

    let dice_roll = 10;

    match dice_roll {
        3 => println!("A three!"),
        7 => println!("A Seven"),
        _ => (),
    }

    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The max is configured to be {max}");
    }
}

fn value_in_cents(coin: Coin) -> u8 {

    // match MUST handle all possibilities
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {

    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
