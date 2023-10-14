fn main() {
    println!("Hello, world!");

    another_function(42, "coucou");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let mut value = get_number();
    value = plus_one(value);
    value = plus_one(value);
    println!("{value}");
}

fn another_function(x : i32, y : &str) {
    println!("{x} - {y}");
}

fn get_number () -> i32 {
    560
}

fn plus_one (x:i32) -> i32 {
    x + 1
}