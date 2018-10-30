struct FizzBuzzTuple {
    fb: String,
    num: u32,
}

impl std::fmt::Display for FizzBuzzTuple {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}:{}", self.fb, self.num)
    }
}

fn main() {
    for i in 1..100 {
        match (i % 3, i % 5) {
            (0, 0) => println!("{}", FizzBuzzTuple{fb: String::from("FizzBuzz"), num: i}),
            (_, 0) => println!("{}", FizzBuzzTuple{fb: String::from("Buzz"), num: i}),
            (0, _) => println!("{}", FizzBuzzTuple{fb: String::from("Fizz"), num: i}),
            _ => (),
        }
    }
}
