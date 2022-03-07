#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
    // todo!("Move `mileage: u32` field into `age` field - a tuple with two fields: an `Age` enum, u32");
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}
#[derive(PartialEq, Debug)]
enum Age {
    New,
    Old,
}
fn car_quality(miles: u32) -> (Age, u32) {
    let quality: (Age, u32) = (Age::New, miles);
    return quality;
}
// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - "age" field calls "car_quality" function with "miles" input argument
    let car: Car = Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    };
    return car;
}
fn main() {
    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Declare the car type and initial values
    let mut car: Car;
    let mut engine: Transmission = Transmission::Manual;
    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[1]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[2]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[3]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
}