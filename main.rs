// use std::fs::File;
// fn main(){
//     let f=File::open("hello.txt").unwrap();

// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main(){
//     let f=File::open("hello.txt");
//     let f=match f{
//         Ok(file) => file,
//         Err(error) => match error.kind(){
//             ErrorKind :: NotFound => match File::create("hello.txt"){
//                 Ok(f) => f,
//             Err(error) => panic!("Error"),
//         }            
            

//       _ => {
//           panic!("NOT WORKING");
//       }
//     }
// };
// }

// Propagating Errors

// use std::fs::File;
// use std::io::{self,Read};

// fn main(){
//     fn read_username() -> Result<String,io::Error>{
//         let f=File::open("hello.txt");
// let mut f=match f{
//     Ok(file) => file,
//     Err(error) => return Err(error),
// };
// let mut s=String::new();
// match f.read_to_string(&mut s){
// Ok(_) => Ok(s),
// Err(error) => Err(error),
// }
//     }
//     println!("{:?}",read_username());
// }

// ERROR PROPOGATION SHORTCUT
// use std::fs::File;
// use std::io::{self,Read};

// fn main(){
//     fn read_username() -> Result<String,io::Error>{

//         let f=File::open("hello.txt")?;
// let mut s=String::new();
// f.read_to_string(&mut s)?;

//     }
//     println!("{:?}",read_username()); 
// }

// PANIC OR NOT TO PANIC
// use std::net::IpAddr;
// fn main(){
//     let home:IpAddr=":1".parse().unwrap();
//    println!("{:?}",assert_eq!(home.is_ipv6(),false));
// }


// Custom validation

// use practise::cal;
// fn main(){
//     cal::its_positive(5);
// }