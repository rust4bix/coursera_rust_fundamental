#[derive(Debug)]

struct Person{
    first_name: String,
    last_name: String,
    age: u8
}

fn main() {
    println!("Hello, week3");
    println!("{:?}", Person{
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25
    });

    let person2 = Person{
        first_name: "Daneil".to_string(),
        last_name: "Kings".to_string(),
        age: 28
    };
    println!("{:?}", person2);
}


/*
Notes:
{:?} is a format specifier used with the println! macro 
to print debug representations of values.

*/