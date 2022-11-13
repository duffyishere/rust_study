fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = 6;
    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("spaces value is: {}", spaces);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
