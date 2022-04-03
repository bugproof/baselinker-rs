
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

    pub mod orders {
        pub mod get_journal_list;
        pub mod add_order;
        pub mod get_orders;
        pub mod get_orders_by_email;
        pub mod get_orders_by_phone;
        pub mod add_invoice;
        pub mod get_order_sources;
        pub mod get_order_transaction_details;
    }
}

pub mod common;
pub mod baselinker;