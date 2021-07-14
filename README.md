# BaseLinker.com API client for Rust

This library is incomplete

## Usage

```rs
let baselinker = BaseLinkerClient::new("API_TOKEN".to_owned(), reqwest::Client::new());
let api_result = baselinker.send(&GetCouriersList {}).await;
match api_result {
    Ok(response) => {
        for courier in response.couriers {
            println!("Courier: {} {}", courier.code, courier.name);
        }
    },
    Err(err) => {
        println!("Error! {} {}", err.code, err.message);
    }
}
```