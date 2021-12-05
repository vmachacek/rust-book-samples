// Procedural macros need importing directly
use rust_decimal_macros::dec;

fn main() {
    let a = [1, 2, 3, 4, 5];

    let a = [3; 8];

    for elem in a {
        println!("{}", elem)
    }
    loop {
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read teh line");

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let a = a[guess];
    }
    // let a: [i32; 5] = [1, 2, 3, 4, 5, 6];

    let tup = (500, 6.4, 1);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let (a, b, c) = tup;

    println!("The value of y is: {}", tup.0);

    let guess: isize = "255".parse().expect("Not a number!");

    let number = 1_000_000.0;
    println!("{}", guess);
    println!("{}", number);

    let number = dec!(-1.23);
    assert_eq!("-1.23", number.to_string());

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
