
enum myEnum {
    Zero,
    One = 10,
    Two,
    Three
}

fn main() {
    println!("{}", myEnum::Zero as u8 );
    println!("{}", myEnum::One as u8);
    println!("{}", myEnum::Two as u8);
    println!("{}", myEnum::Three as u8);
}
