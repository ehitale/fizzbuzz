fn main() {
    fb();
}

fn fb() {
    //fizzing and buzzing
    for x in 1..101 {
        let divis_by_three = x % 3;
        let divis_by_five = x % 5; 

        let y = (divis_by_three, divis_by_five);

        match y {
            (0 , 0) => println!("fizzbuzz"),
            (0 , _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", x),
        };
    }
}
