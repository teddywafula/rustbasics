fn main() {
    // if statement

    let num = 10;
    if num == 10 {
        println!("{} the correct value", num);
    }
    // if else
    if num < 10 {
        println!("The value is less than {}", num);
    } else {
        println!("{} is greater or equal to 10", num);
    }

    // if else nested statement
    if num < 10 {
        println!("{} is less than 10", num);
    } else if num > 10 {
        println!("{} is greater than 10", num);
    } else {
        println!("{} is equal to 10", num);
    }

    let counties = "NRB";
    println!("{}", counties);
    let c = match counties {
        "MBS" => "Mombasa",
        "NRB" => "Nairobi",
        _ => "Unknown"
    };
    println!("The county is {}", c);
}
