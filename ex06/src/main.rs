fn main() {
    println!("{}", Coin::Quarter(UsState::Alabama).value_in_cents());

    let some_u8_value = Some(3);

    if let Some(3) = some_u8_value {
        println!("This is number 3.");
    } else {
        println!("else")
    }
}

#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Us state: {:?}", state);
                25
            }
        }
    }
}