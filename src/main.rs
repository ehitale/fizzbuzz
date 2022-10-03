struct Divisable {
    by_three: bool, 
    by_five: bool,
}

enum Divis {
    ByFifteen(i32),
    ByThree(i32),
    ByFive(i32),
    ByNOTB(i32), 
}

fn main() {
    //base fizzbuzz, no strings attached.
    fb();
    println!("--------");

    //fizzbuzz with structs
    fb_w_structs();
    println!("--------");
    
    //fizzbuzz with enums
    fb_w_enums();
    println!("--------");
}

fn fb() {
    //fizzing and buzzing
    for foo in 0..100 {
    let divis_by_three = (foo + 1) % 3;
        let divis_by_five = (foo + 1) % 5; 

        let bar = (divis_by_three, divis_by_five);

        match bar {
            (0 , 0) => println!("fizzbuzz"),
            (0 , _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", foo + 1),
        };
    };
}

fn fb_w_structs() {
    // I'm just going to copy and paste some code I already did in Repl.it
    for foo in 0..100 {
    
        let bar = Divisable {
            by_three: if ((foo + 1) % 3) == 0 {true} else {false},
            by_five: if ((foo + 1) % 5) == 0 {true} else {false},
        };
        
        match (bar.by_three , bar.by_five) {
            (true, true) => println!("Fizzbuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            _ => println!("{}", foo + 1),
        }
    }
}

fn fb_w_enums() {
    for foo in 0..100 {
        match int_to_divis(foo) {
            Divis::ByFifteen(n) => println!("{}: Fizzbuzz", n), 
            Divis::ByThree(n) => println!("{}: Fizz", n), 
            Divis::ByFive(n) => println!("{}: Buzz", n),
            Divis::ByNOTB(n) => println!("{}: no ruckus at all!", n),
        }
    };
}

fn int_to_divis (foo: i32) -> Divis {
    let divis_by_three = (foo + 1) % 3;
    let divis_by_five = (foo + 1) % 5;  
    
    let y = (divis_by_three, divis_by_five);

    match y {
        (0, 0) => Divis::ByFifteen(foo + 1),
        (0, _) => Divis::ByThree(foo + 1),
        (_, 0) => Divis::ByFive(foo + 1),
        _ => Divis::ByNOTB(foo + 1),
    }
}