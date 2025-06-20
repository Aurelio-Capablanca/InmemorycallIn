use std::io;

pub(crate) fn validate_integers(input : &mut String, text: &str) -> i32{
    let mut value : i32  = 0;
    println!("{}",text);
    io::stdin().read_line(input).unwrap();
    match input.trim().parse::<i32>(){
        Ok(num) => {
            value = num;
        },
        Err(e) => {
            println!("You can't submit a non Integer value!");
            validate_integers(input, "Re insert the values : ");
        }
    };
    value
}