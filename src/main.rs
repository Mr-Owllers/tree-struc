use std::io;
mod colours;

fn main() {
    let green = colours::green;
    let red = colours::red;
    let reset = colours::reset;
    println!("run how many times? (max = 7) - ");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read line");
    let a: u32 = inp.trim().parse().expect("Please type a number!");
    if a > 7 {
        println!("{red}ERROR: MAX INT = 7{reset}");
        return;
    }
    let mut power = i64::pow(2, a);
    print!("{green}");
    for h in 1..a + 1 {
        for _h in 1..power+1 {
            print!(" ");
        }
        for _leaf in 1..h + 1{
            print!("/\\");
        }
        print!("\n");
        power -= 1;
    }
    print!("{red}");
    power = i64::pow(2, a);
    for _h in 1..4 {
        for _h in 1..power+1 {
            print!(" ");
        }
        println!("||");
    }
    print!("{reset}");
}