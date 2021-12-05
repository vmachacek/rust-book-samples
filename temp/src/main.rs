fn main() {
    let mut choise: String;

    loop {
        println!("Choose convertsion:");
        println!("\t F: F -> C");
        println!("\t C: C -> F");
        choise = String::new();
        std::io::stdin()
            .read_line(&mut choise)
            .expect("Failed to read the line");

        choise = choise.trim().to_lowercase().to_string();

        println!("choise: {}", choise);

        if choise.eq("f") || choise.eq("c") {
            break;
        }
    }

    loop {
        println!("Enter number");

        let mut temp_str = String::new();

        std::io::stdin()
            .read_line(&mut temp_str)
            .expect("Failed to read the line");

        let number: f32 = match temp_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choise == "f" {
            let result = (number - 32.0) * 5.0 / 9.0;
            println!("Temperature of {}F is {} in C", number, result);
        }

        if choise == "c" {
            let result = number * 1.8 + 32.0;
            println!("Temperature of {}C is {} in F", number, result);
        }
        break;
    }
}
