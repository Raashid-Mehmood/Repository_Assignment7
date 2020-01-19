// mod counting {
//     pub mod check {
//         pub fn digits() {
//             println!("We are in check function of counting module - BINARY PACKAGE");
//             for counting in 1..11 {
//                 println!("{}",counting);
//             }
//         }
//     }
// }


                                // making BINARY PACKAGE main.rs to use lib.rs for execution in bin package
use assignment7_q3;             //using library in main.rs             

fn main() {
    // println!("Calling of mod in fn main function");
    // crate::counting::check::digits();

    assignment7_q3::counting3::check::digits();     //calling library package function in fn main()
}
