
pub mod requests {
    pub mod courier_shipments {
        pub mod create_package;
        pub mod create_package_manual;
        pub mod get_couriers_list;
        pub mod get_courier_fields;
        pub mod get_courier_services;
        pub mod get_courier_accounts;
        pub mod get_label;
        pub mod get_order_packages;
        pub mod get_courier_packages_status_history;
        pub mod delete_courier_package;
    }

    pub mod external_storages {
        pub mod get_external_storages_list;
        pub mod get_external_storage_categories;
        pub mod get_external_storage_products_data;
        pub mod get_external_storage_products_list;
        pub mod get_external_storage_products_quantity;
        pub mod get_external_storage_products_prices;
        pub mod update_external_storage_products_quantity;
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