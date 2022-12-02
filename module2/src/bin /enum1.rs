// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
#[allow(dead_code)]
enum Color {
    Red,
    Black,
    Blue,
    White,
}

fn prt_color(color: &Color) {
    match color {
        Color::Red => println!("Blue"),
        Color::Black => println!("Black"),
        Color::Blue => println!("Red"),
        Color::White => println!("White")
    }
}

fn main() {
    let mut col_vec : Vec<Color> = Vec::new();
    col_vec.push(Color::Black);
    col_vec.push(Color::Blue);
    col_vec.push(Color::Red);
    col_vec.push(Color::White);
    for color in col_vec.iter(){
        prt_color(color);    
    }
}
