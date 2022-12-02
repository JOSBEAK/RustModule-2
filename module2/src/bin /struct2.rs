//
//Requirements:
/*
 1-Define  struct Shape having  square:Square and rectangle:Rectangle
 2-Square has one field as side:i32
 3-Rectangle has two length:i32 and width:i32
 4-Create a function which returns a new Rectangle
 5-Create a function which returns a new Square
 6-Create a function which returns a new Shape
 7-Create a function which takes Shape and prints the dimension of all shapes
*/

struct Shape {
    square: Square,
    rectangle: Rectangle,
}
struct Square {
    side: i32,
}
struct Rectangle {
    length: i32,
    width: i32,
}

fn new_square(side: i32) -> Square {
    let sqr = Square { side: side };
    sqr
}

fn new_rectangle(length: i32, width: i32) -> Rectangle {
    let rec = Rectangle {
        length: length,
        width: width,
    };
    rec
}

fn new_shape(square: Square, rectagle: Rectangle) -> Shape {
    let shp = Shape {
        square: square,
        rectangle: rectagle,
    };
    shp
}
fn print_shape(shape: Shape) {
    println!(" Side of the square is = {:?} \n length of the rectangle is = {:?} \n width of the rectangle is = {:?}", shape.square.side, shape.rectangle.length, shape.rectangle.width);
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let side = buf.trim().parse::<i32>().unwrap();
    let n_sqr = new_square(side);

    let mut buf1 = String::new();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();
    let length = buf1.trim().parse::<i32>().unwrap();
    let width = buf2.trim().parse::<i32>().unwrap();
    let n_rec = new_rectangle(length, width);

    let n_shape = new_shape(n_sqr, n_rec);

    print_shape(n_shape)
}
//Done