
pub mod requests {
    pub mod courier_shipments {
        pub mod get_couriers_list;
        pub mod create_package;
        pub mod create_package_manual;
    }
}

pub mod common;
pub mod baselinker;

#[cfg(test)]
mod tests {
    use crate::requests::courier_shipments::get_couriers_list::GetCouriersList;
    use crate::baselinker::BaseLinkerClient;

    #[tokio::test]
    async fn it_works() {
        // not really a test just was lazy to create an example console app
        let baselinker = BaseLinkerClient::new("3001646-3007387-D7KZKSX8A2JFJEX41KG9YNBCQEPLYBHKRVSNM4QC0DWSN29226CZVVDSGR6A5WR6".to_owned(), reqwest::Client::new());
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

        assert_eq!(2 + 2, 4);
    }
}