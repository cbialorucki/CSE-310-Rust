use std::io;
use std::collections::LinkedList;
use std::collections::HashMap;

struct User{
    name: String,
    email: String,
    favorite_color: String,
}

fn main() {
    println!("Hello, world! Welcome to my program.");
    println!("What is your name?");
    let name = get_input();
    println!("Ah, {}. Nice to meet you {}!", name, name);
    println!("Let me calculate something for you...");
    
    loop{
        println!("Would you like to add, subtract, multiply, or divide?");
        let input = get_input().to_lowercase();

        if input == "add" || input == "+"{
            println!("Your numbers added together equal {}.", calc('+'));
            break;
        }

        else if input == "subtract" || input == "-"{
            println!("The difference between your numbers is {}.", calc('-'));
            break;
        }

        else if input == "multiply" || input == "x"{
            println!("Your numbers multiplied together equal {}.", calc('*'));
            break;
        }

        else if input == "divide" || input == "/"{
            println!("The quotient between your numbers is {}.", calc('/'));
            break;
        }
    }
    
    println!("So {}, let's get you set up in a user account.", name);
    println!("What's your email address?");
    let email = get_input();
    println!("And what's your favorite color?");
    let fav_color = get_input();

    let current_user = User {
        email: String::from(email),
        name: String::from(name),
        favorite_color: fav_color,
    };

    println!("Perfect! I made an account for you! Here are the details.");
    println!("\nEmail: {}", current_user.email);
    println!("Name: {}", current_user.name);
    println!("Favorite Color: {}\n", current_user.favorite_color);

    let vec = vec![38, 78, 3, 96, 91, 3, 42, 89, 41, 58];

    println!("Here's a slice of a list of numbers from a vector.");
    println!("{:?}",&vec[3..7]);

    println!("\nAnd here's the list of numbers that were stored in that vector.");
    println!("{:?}",&vec);

    let linked_list = LinkedList::from([16, 70, 42, 59, 24, 38, 23, 33, 83, 9]);
    println!("\n...And a linked list.");
    println!("{:?}",&linked_list);

    let hash_map = HashMap::from([
        ("Meaning of Life", 42),
        ("My Favorite Number", 32),
        ("MPG on a diesel 2004 Chevy Silverado 2500", 22),
        ("Fastest speed limit in the US (in MPH)", 85),
        ("Population of the United States", 326_690_000),
    ]);
    println!("\nFinally, a hash map.");
    println!("{:?}",&hash_map);

    println!("\nWell, I think we're about done here. Have a great day, {}!", current_user.name);
}

fn get_input() -> String{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn calc(term: char) -> i32{
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;
    
    loop{
        println!("Enter the first number");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

            num1 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        break;
    }

    loop{
        println!("Enter the second number");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

            num2 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            
        break;
    }
    
    if term == '+' { num1 + num2 }
    else if term == '-' { num1 - num2 }
    else if term == '*' { num1 * num2 }
    else if term == '/' {num1 / num2 }

    // Should never happen, added to appease compiler.
    else { 0 }

}
