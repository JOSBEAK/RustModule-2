// Use a enum with variants as different car names and
// create a function which accepts an &str, 
// using match statement return the correct enum variant
// define a struct Details 
// car:Car
//price:i32

enum Car {
    Hyundai,
    Honda,
    Suzuki,
    Jaguar
}
struct Details{
    car:&str,
    price:i32
}
fn vehicle(info:&str){
    match info {
        "Hyundai" => println!("Hyundai"),
        "Honda" => println!("Honda"),
        "Suzuki" => println!("Suzuki"),
        "Jaguar" => println!("Jaguar")
    }
}


fn main(){
    let veh = Details {
        car: "Jaguar",
        price: 20000000,
    };
    vehicle(veh.car);
}


