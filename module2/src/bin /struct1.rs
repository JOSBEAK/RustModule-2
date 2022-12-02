// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces

// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

#[derive(Debug)]
enum Flavor {
    BlueBerry,
    Cola,
    Mojito,
}
#[derive(Debug)]
struct Drink {
    flavour: Flavor,
    ounces: f32,
}
fn print_flavour(flav: Flavor) -> String {
    match flav {
        Flavor::BlueBerry => String::from("BlueBerry"),
        Flavor::Cola => String::from("Cola"),
        Flavor::Mojito => String::from("Mojito"),
    }
}
fn print_data(data: Drink) {
    println!("{:?} {:?}", print_flavour(data.flavour), data.ounces);
}
fn main() {
    let mock_tail1 = Drink {
        flavour: Flavor::BlueBerry,
        ounces: 24.05,
    };
    let mock_tail2 = Drink {
        flavour: Flavor::Cola,
        ounces: 42.5,
    };
    // println!("{:?}", mock_tail1.flavour);
    print_data(mock_tail1);
    print_data(mock_tail2);
}
//Done