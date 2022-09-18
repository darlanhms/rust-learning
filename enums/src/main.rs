/**
 * Option enum
 */
// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     dbg!(five, six, none);
// }

/**
 * Catch-all Patterns and the _ Placeholder 
 */

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll(),
//     }
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn reroll() {}

// fn main() {
//     let dice_roll = 9;

//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => (),
//     }
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}

/**
 * If let syntax
 */
// fn main() {
//     let config_max = Some(3u8);

//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {}", max);
//     } else {
//         println!("The maximum is not configured");
//     }
// }

fn main() {
    
}