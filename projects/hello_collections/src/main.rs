use std::collections::HashMap;

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Text(String),
    Float(f64),
}

fn collection_examples() {
    let mut v: Vec<i32> = Vec::new();
    let v_macro = vec![1, 2, 3];
    v.push(4);
    v.push(5);

    println!("{:?}", v_macro);
    println!("{:?}", v);

    let val_by_ref: &i32 = &v[1]; // panics on out of bound index

    // v.push(6); // Rustc complains about mutable borrow

    let val_by_get: Option<&i32> = v.get(2); // returns None on Out of bound index

    println!("{:?}", val_by_ref); // immutable borrow

    match val_by_get {
        Some(_val) => println!("{:?}", val_by_get),
        None => println!("Array index out of bound."),
    }

    // loop through vector
    for i in &v {
        println!("{i}");
    }

    // iterate over mutable reference
    let mut v2: Vec<i32> = vec![1, 2, 3];
    for i in &mut v2 {
        *i += 50; // To learn more about pointers
        println!("{i}");
    }

    v.push(6); // Rutc does not complain about mutable borrow here as immutable borrow has ended in scope

    let sheet = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("Hello")),
        SpreadSheetCell::Float(0.1),
    ];

    for cell in &sheet {
        match cell {
            SpreadSheetCell::Int(value) => println!("Integer: {}", value),
            SpreadSheetCell::Text(value) => println!("Text: {}", value),
            SpreadSheetCell::Float(value) => println!("Float: {}", value),
        }
    }
}

fn string_examples() {
    let mut s: String = String::new();
    s.push_str("foo");
    s.push_str(" ");
    s.push_str("bar");

    println!("{s}");

    let s1: String = String::from("hello");
    let s2: String = String::from("world");

    let s = s1 + &s2;

    println!("{s}");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let mut output = format!("**--{tic}--{tac}--{toe}--**");

    output.push_str("--done");

    println!("{output}");

    // let h = tic[0]; // Accessing strings by index errors in rust

    // println!("{h}");

    // explicitly as for chars representation for iteration
    for c in "Зд".chars() {
        println!("{c}");
    }

    // explicitly as for bytes representation for iteration
    for c in "Зд".bytes() {
        println!("{c}");
    }
}

fn hash_map_examples() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 100);

    println!("{:?}", scores);

    let team = String::from("red");
    let points = scores.get(&team).copied().unwrap_or(0); // get returns Option<&V>

    println!("{:?}", points);

    scores.entry(String::from("blue")).or_insert(50);
    scores.entry(String::from("green")).or_insert(50);
    scores.entry(String::from("black")).or_insert(50);

    // iterate over values
    for (key, val) in &scores {
        println!("{}, {}", key, val);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

fn my_vanilla_solution_for_finding_mean_and_mode() {
    let mut vectors = Vec::new();
    vectors.push(1);
    vectors.push(10);
    vectors.push(1);
    vectors.push(100);
    vectors.push(5);
    vectors.push(5);
    vectors.push(5);
    vectors.push(5);

    vectors.sort();

    let total = vectors.len();
    println!("{vectors:?}");
    println!("{total:?}");

    let median = vectors[total / 2];
    println!("median: {median:?}");

    let mut count_map = HashMap::new();
    for num in vectors {
        *count_map.entry(num).or_insert(0) += 1;
    }

    let mut max_value = 0;
    let mut max_key = 0;
    for (key, val) in &count_map {
        let v = *val;
        let k = *key;
        if v >= max_value {
            max_value = v;
            max_key = k;
        }
    }

    println!("{count_map:?}");
    println!("Mode: {max_key:?} with {max_value:?}");
}

fn calculate_mean_and_mode(numbers: &mut Vec<i32>) -> (f64, i32) {
    numbers.sort();

    // median calculation
    // Median calculation
    let median = if numbers.len() % 2 == 0 {
        // Even length: average of the two middle numbers
        let mid1 = numbers[numbers.len() / 2 - 1];
        let mid2 = numbers[numbers.len() / 2];
        (mid1 + mid2) as f64 / 2.0
    } else {
        // Odd length: middle number
        numbers[numbers.len() / 2] as f64
    };

    // mode calculation
    let mut frequency_map = HashMap::new();
    for &num in numbers.iter() {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    // need to learn standard library well
    let mode = frequency_map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
        .unwrap_or(0); // Default to 0 if the list is empty

    (median, mode)
}

fn challenge_1() {
    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    my_vanilla_solution_for_finding_mean_and_mode();

    let mut numbers = vec![1, 2, 2, 3, 4, 4, 4, 5, 5];
    let (median, mode) = calculate_mean_and_mode(&mut numbers);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

// failed exercise, strings are hard in Rust!
// messed up string completely!
fn to_pig_latin(input: &str) -> String {
    let mut result = String::new();
    for word in input.split_whitespace() {
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            if is_vowel(first_char) {
                // Word starts with a vowel
                let formatted_str = format!("{}-hay ", word);
                result.push_str(&formatted_str);
            } else if first_char.is_alphabetic() {
                // Word starts with a consonant
                let rest: String = chars.collect();
                let formatted_str = format!("{}-{}ay ", rest, first_char);
                result.push_str(&formatted_str);
            } else {
                println!("Not a vowel or consonant!");
            }
        }
    }
    result.trim_end().to_string()
}

fn to_pig_latin_with_map_collect_join(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars(); // Get an iterator over the characters
            if let Some(first_char) = chars.next() {
                if is_vowel(first_char) {
                    // Word starts with a vowel
                    format!("{}-hay", word)
                } else {
                    // Word starts with a consonant
                    let rest: String = chars.collect();
                    format!("{}-{}ay", rest, first_char)
                }
            } else {
                // Handle empty word (unlikely with split_whitespace)
                String::new()
            }
        })
        .collect::<Vec<_>>() // Collect the transformed words into a Vec
        .join(" ")
}

fn challenge_2() {
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
    let input = "first apple orange";
    let pig_latin = to_pig_latin(input);
    let pig_latin_with_map_collect_join = to_pig_latin_with_map_collect_join(input);
    println!("Pig Latin: {pig_latin}");
    println!("Pig Latin with Map, Collect, Join: {pig_latin_with_map_collect_join}");
}

struct Company {
    department: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Company {
            department: HashMap::new(),
        }
    }

    fn add_user_to_department(&mut self, user: &str, dept: &str) {
        self.department
            .entry(dept.to_string())
            .or_insert(Vec::new())
            .push(user.to_string());
    }

    fn all_people_in_a_department(&mut self, dept: &str) -> Vec<String> {
        let mut employees = self.department.get(dept).cloned().unwrap_or(Vec::new());
        employees.sort();
        employees
    }

    fn all_people_in_a_company_by_department(&mut self) -> HashMap<String, Vec<String>> {
        let mut sorted_hashmap:HashMap<String, Vec<String>>  = HashMap::new();
        for (dept, employees) in &self.department {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            sorted_hashmap.insert(dept.to_string(), sorted_employees);
        }
        sorted_hashmap
    }
}

fn challenge_3() {
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

    let mut company = Company::new();

    company.add_user_to_department("tom", "finance");
    company.add_user_to_department("sham", "finance");
    company.add_user_to_department("jack", "engineeering");
    company.add_user_to_department("steve", "design");

    let dept_people: Vec<String> = company.all_people_in_a_department("finance");

    println!("Dept People: {dept_people:?}");

    let company_people: HashMap<String, Vec<String>> =
        company.all_people_in_a_company_by_department();

    println!("Company People {company_people:?}");
}

fn main() {
    println!("Hello, world!");

    collection_examples();

    string_examples();

    hash_map_examples();

    challenge_1();

    challenge_2();

    challenge_3();

    // Needs a ton more practice!
}
