use baselinker::{BaseLinkerClient, Error};
use baselinker::requests::courier_shipments::get_couriers_list::GetCouriersList;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("BASELINKER_TOKEN").expect("BASELINKER_TOKEN environment variable");
    let mut baselinker = BaseLinkerClient::new(token, reqwest::Client::new());
    let api_result = baselinker.send(&GetCouriersList {}).await;
    match api_result {
        Ok(response) => {
            for courier in response.couriers {
                println!("{:?}", courier);
            }
        }
        Err(err) => match err {
            Error::BaseLinkerError(baselinker_error) => {
                println!("{:?}", baselinker_error);
            }
            Error::NetworkError(_) => {}
        },
    }

    Ok(())
}
