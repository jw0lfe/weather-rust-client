use std::collections::HashMap;

use reqwest::header::AUTHORIZATION;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
      
    //https://docs.rs/reqwest/latest/reqwest/
    let mut map = HashMap::new();
    map.insert("username", "jabob");
    map.insert("password", "D0nChaKn0w");

    let client = reqwest::Client::new();
    let auth_response = client
        .post("http://ec2-52-12-191-147.us-west-2.compute.amazonaws.com:3000/v1/auth")
        .json(&map)
        .send()
        .await?;

    let authToken = auth_response
        .json::<model::Token>()
        .await?;
    println!("\nThe token is:\n {:?}", authToken.access_token);

    //https://stackoverflow.com/questions/66534704/how-to-add-basic-authorization-header-by-passing-the-secret-using-reqwest
    let header = "Bearer".to_owned() + &authToken.access_token;

    let greeting_response = client
        .get("http://ec2-52-12-191-147.us-west-2.compute.amazonaws.com:3000/v1/hello")
        .header(AUTHORIZATION,header.clone())
        .send()
        .await?;

    let greeting = greeting_response
        .json::<model::Greeting>()
        .await?;

    println!("\nWeather from openweathermap.org:\n {:?}", greeting);

    let weather_response = client
        .get("http://ec2-52-12-191-147.us-west-2.compute.amazonaws.com:3000/v1/weather")
        .header(AUTHORIZATION,&header)
        .send()
        .await?;

    let weather = weather_response
        .json::<model::Weather>()
        .await?;

    println!("\nWeather from openweathermap.org:\n {:?}", weather);

    Ok(())
}
