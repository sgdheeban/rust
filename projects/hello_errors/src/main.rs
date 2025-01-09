use std::{
    error::Error, fs::File, //io::ErrorKind
};

/** All errors handled within the function */
/*fn examples_of_error_handling() {
    println!("Hello, world!");
    // panic!("panic by default!"); // explicitly call panic! macro

    //let v:Vec<i32> = vec![1,2,3];
    //v[99];

    let greeting_file_result = File::open("src/hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // println!("{error:?}");
            // panic!("panic!");
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // helper - unwrap method
    // let greeting_file = File::open("hello.txt").unwrap();
    //let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");

    print!("{greeting_file:?}");
}*/

/** Propogate error from a function */
/*fn read_user_name_from_file() -> Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    }; // explicit return to shortcircuit the function, semicolon

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    } // implicit return as this is the last line of the function, no semicolon
}*/

/** Propogate error from a function */
/*fn read_user_name_from_file_with_shortcut() -> Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    }; // explicit return to shortcircuit the function, semicolon

    let mut username = String::new();

    username_file.read_to_string(&mut username)?; // ? shortcut operator, re-throwing error

    Ok(username)
}*/

/** Propogate error from a function */
/*fn read_user_name_from_file_with_shortcut_2() -> Result<String, std::io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn handle_error_from_main() {
    examples_of_error_handling();

    let result = read_user_name_from_file_with_shortcut_2();

    print!("{result:?}");

    match result {
        Ok(res) => print!("main-ok: {res:?}"),
        Err(e) => print!("main-err: {e:?}"),
    };
}*/

// I naturally understood this line as I typed ☺️
/*fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}*/

fn main() -> Result<(), Box<dyn Error>> {
    // handle_error_from_main();
    File::open("hello.txt")?;
    Ok(())
}
