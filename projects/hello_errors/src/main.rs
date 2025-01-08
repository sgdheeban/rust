use std::fs::File;

fn main() {
    println!("Hello, world!");
    // panic!("panic by default!"); // explicitly call panic! macro

    //let v:Vec<i32> = vec![1,2,3];
    //v[99];

    let greeting_file_result = File::open("src/hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            println!("{error:?}");
            panic!("panic!");
        }
    };

    print!("{greeting_file:?}");

}
