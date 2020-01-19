// mod counting {
//     pub mod check {
//         pub fn digits() {
//             println!("We are in check function of counting mod");
//             for counting in 1..11 {
//                 println!("{}",counting);
//             }
//         }
//     }
// }


mod lib;    // using lib.rs file

fn main() {
    // println!("Calling of mod in fn main function");
    // crate::counting::check::digits();

    lib::welcome::checking::digits();       //calling sub-module function named digits() in main.rs for execution
}
