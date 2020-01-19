pub mod welcome {               //lib.rs file creation alogwith main.rs; and making welcome module
    pub mod checking {          //sub-module creation in library
        pub fn digits() {       //defining simple function of counting in sub-module
            println!("Execution of  lib.rs file, digits function through main.rs");
            for counting2 in 1..11 {
                println!("{}",counting2);
            }
        }
    }
}
