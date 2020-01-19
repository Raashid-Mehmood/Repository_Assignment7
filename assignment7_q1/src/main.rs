mod counting {                              // MODULE CREATION
    pub mod check {                         //SUB MODULE CREATION IN MAIN MODULE
        pub fn digits() {                   //DEFINING A FUNCTION IN SUB MODULE
            println!("We are in check function of counting module");
            for counting in 1..11 {
                println!("{}",counting);
            }
        }
    }
}


fn main() {
    println!("Calling of module in fn main function");
    crate::counting::check::digits();      //CALLING OF FUNCTION FROM fn main() - ABSOLUTE PATH
}
