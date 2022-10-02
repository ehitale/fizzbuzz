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
    #[allow(dead_code)]
    fb_w_structs();
    println!("--------");
    
    //fizzbuzz with enums
    #[allow(dead_code)]
    fb_w_enums();
    println!("--------");
}

fn fb() {
    //fizzing and buzzing
    for x in 0..100 {
        let divis_by_three = (x + 1) % 3;
        let divis_by_five = (x + 1) % 5; 

        let y = (divis_by_three, divis_by_five);

        match y {
            (0 , 0) => println!("fizzbuzz"),
            (0 , _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", x + 1),
        };
    };
}

fn fb_w_structs() {
    // I'm just going to copy and paste some code I already did in Repl.it
}

fn fb_w_enums() {
    for zed in 0..100 {


        {
            match int_to_divis(zed) {
                Divis::ByFifteen(n) => println!("{}: Fizzbuzz", n), 
                Divis::ByThree(n) => println!("{}: Fizz", n), 
                Divis::ByFive(n) => println!("{}: Buzz", n),
                Divis::ByNOTB(n) => println!("{}: no ruckus at all!", n),
            }
        }

    };
}

fn int_to_divis (zed: i32) -> Divis {
    let divis_by_three = (zed + 1) % 3;
    let divis_by_five = (zed + 1) % 5;  
    
    let y = (divis_by_three, divis_by_five);

    match y {
        (0, 0) => Divis::ByFifteen(zed + 1),
        (0, _) => Divis::ByThree(zed + 1),
        (_, 0) => Divis::ByFive(zed + 1),
        _ => Divis::ByNOTB(zed + 1),
    }
}