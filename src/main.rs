use hello::greet;

fn main() {

    let result =  do_stuff(2.3,3.4);
    println!("RESULT {}", result); 

    greet();
}


fn do_stuff(qty: f64, oz: f64) -> f64{
     qty * oz
}


fn previous(qty: f64, oz: f64) { 

    // VARIABLE
    println!("Hello, world!");



    let mut cars = 2;
    cars = 34;

    const CAR_TEST: f64 = 9.8;


    // SCOPES
    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y)
    }


    //SCOPE 2 
    let x = 5;
    {
        let x = 99;
        println!("{}", x) // Prints 99
    }

    // MEMORY SAFETY
    let enigma: i32;
    if true {
        enigma = 42;
    }

    ///////
    let enigma: i32;
    if true {
        enigma = 42;
    } else {
        enigma = 12; 
    }
    println!("{}", enigma);


}