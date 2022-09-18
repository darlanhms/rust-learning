use std::io;

/**
 * 
 * Basic ownership concept with &
 * 
 */
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, it is not dropped.

/**
 * 
 * String ownership and slices
 * 
 */

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     let my_string = String::from("hello world");

//     // `first_word` works on slices of `String`s, whether partial or whole
//     let word = first_word(&my_string[0..6]);
//     let word = first_word(&my_string[..]);
//     // `first_word` also works on references to `String`s, which are equivalent
//     // to whole slices of `String`s
//     let word = first_word(&my_string);

//     let my_string_literal = "hello world";

//     // `first_word` works on slices of string literals, whether partial or whole
//     let word = first_word(&my_string_literal[0..6]);
//     let word = first_word(&my_string_literal[..]);

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
// }

// fn main() {
//     println!("Please input your words.");
   
//     let mut words = String::new();

//     io::stdin()
//         .read_line(&mut words)
//         .expect("Failed to read line");

//     let word = first_word(&words);

//     println!("The first word is '{}'", word);
// }


/**
 * 
 * Array slices
 * 
 */
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("{}", &a[1]);
//     println!("{}", &a[3]);

//     let slice = &a[1..3];

//     assert_eq!(slice, &[2, 3]);
// }

fn main() {
    
}