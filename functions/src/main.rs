fn main() {
    println!("The value of y is: {}", yolo(12, 14) + five());
}

fn yolo(s: i32, g: i32) -> i32 {
     s + g 
}

fn five() -> i32 {
    5
}


fn plus_one(x: i32) -> i32 {
    x + 1;
}