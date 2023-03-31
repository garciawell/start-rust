fn main() {

    // VARIABLE
    println!("Hello, world!");

    let number = 16;

    let mut cars = 2;
    cars = 34;

    const CAR_TEST: f64 = 9.8;


    // SCOPES
    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y)
    }
    println!("{}, {}", x, y) // ERROR

    //SCOPE 2 
    let x = 5;
    {
        let x = 99;
        println!("{}", x) // Prints 99
    }
    println!("{}", x) // Prints 5


    // MEMORY SAFETY
    let enigma: i32;
    if true {
        enigma = 42;
    }
    println!("{}", enigma) // ERROR you must to initialize variable

    ///////
    let enigma: i32;
    if true {
        enigma = 42;
    } else {
        enigma = 12; 
    }
    println!("{}", enigma) // WORKED

}
