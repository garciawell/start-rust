pub fn greet() {
    println!("Hi!");

    print_noise(23);
}


struct RedFox {
    enemy: bool,
    life: u32
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
   fn get_noise(&self) -> &str { "Ua Uau!" }
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str { "BYTE" }
 }

 fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
 }