use std::{collections::HashMap, io};

use futures::executor::block_on;

#[tokio::main]
async fn main() {
    block_on(main_async());
}

async fn main_async() {
    let res = get().await;

    let res = match res {
        Ok(v) => v,
        Err(e) => {
            panic!("Failure to fetch data: {:?}", e);
        }
    };

    println!("first: {} - second: {}", res["key1"], res["key2"]);
    // res.iter().for_each(|(k, v)| println!("{}: {}", k, v));

    //~ select an operator
    println!("Select an operator - '+', '-', '*', '/'");
    let mut operator_input = String::new();
    io::stdin().read_line(&mut operator_input).unwrap();

    //~ enter numeric equivalent of worded numbers
    println!("Enter a numeric equivalent of the first input");

    let mut user_inputs: [String; 3] = [String::new(), String::new(), String::new()];
    let what_inputs = vec!["First number", "Operator", "Second number"];

    for i in 0..user_inputs.len() {
        collect_data(&mut user_inputs[i], &what_inputs[i]);
    }

    println!(
        "Your inputs: {} {} {}",
        user_inputs[0], user_inputs[1], user_inputs[2]
    );

    
}

fn collect_data(input: &mut String, what_input: &str) {
    println!("{}:", what_input);
    io::stdin().read_line(input).unwrap();
}

// 1. fetch
// 2. have the user convert
// 3. send answer

// 4. display on screen
// 5. add a ... loading indicator

async fn get() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    //? Result, Box dyn std::error::Error
    let resp = reqwest::get("https://100insure.com/mi/api1.php")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(resp)
}
// println!("body = {:?}", body)

// fn converter() {}

async fn post() {
    //? Result, Box dyn std::error::Error
    let client = reqwest::Client::new();

    let resp = client.post("https://100insure.com/mi/api2.php")
        .body("the exact body that is sent")
        .send()
        // .json::<HashMap<String, String>>()
        .await;
}
