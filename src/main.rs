

// refrence borrow

// refrence borrow

// refrence borrow


// fn main () {
// let s = String::from("hello");
// let s2 = calculate_length(&s);
// println!("{}", s2);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// mut refrence
// fn main () {
// let mut s = String::from("hello");
// change(&mut s);
// println!("{}", s);
// }

// fn change(s: &mut String) {
//     s.push_str(", world");
// }


// slice type 

// fn main() {
//     let s = String::from("hello world");
//     let hello = &s[0..5];
//     let world = &s[6..11];
//     s.clear();  //error
//     println!("{} {}", hello, world);
// }





//struct 


// struct  Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!("rect1 is {} x {}" , rect1.width , rect1.height); 
// }


// tupple struct 
// #[derive(Debug)]
// struct Rectangle (u32, u32);

// fn main() {
//     let rect1 = Rectangle(30, 50);
//     println!("rect1 is {} x {} and rectangle is {:?} ", rect1.0 , rect1.1 , rect1); 
// }


//method syntax