pub mod counting3 {                 //making lib.rs package through cargo new package_name --lib, and making module
    pub mod check {                 //making sub-module
        pub fn digits() {           //defining function in sub-module
            println!("We are in lib.rs package");
            for counting in 1..9 {
                println!("{}",counting);
            }
        }
    }
}
