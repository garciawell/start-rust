const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let  mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;

    missiles = missiles - ready;


    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles);
}
