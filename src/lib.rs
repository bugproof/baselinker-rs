mod serialization;

pub mod requests {
    pub mod courier_shipments {
        pub mod create_package;
        pub mod create_package_manual;
        pub mod delete_courier_package;
        pub mod get_courier_accounts;
        pub mod get_courier_fields;
        pub mod get_courier_packages_status_history;
        pub mod get_courier_services;
        pub mod get_couriers_list;
        pub mod get_label;
        pub mod get_order_packages;
        pub mod get_protocol;
        pub mod get_request_parcel_pickup_fields;
        pub mod request_parcel_pickup;
    }

    pub mod external_storages {
        pub mod get_external_storage_categories;
        pub mod get_external_storage_products_data;
        pub mod get_external_storage_products_list;
        pub mod get_external_storage_products_prices;
        pub mod get_external_storage_products_quantity;
        pub mod get_external_storages_list;
        pub mod update_external_storage_products_quantity;
    }

    pub mod orders {
        pub mod add_invoice;
        pub mod add_order;
        pub mod add_order_invoice_file;
        pub mod add_order_product;
        pub mod delete_order_product;
        pub mod get_invoices;
        pub mod get_journal_list;
        pub mod get_new_receipts;
        pub mod get_order_payments_history;
        pub mod get_order_sources;
        pub mod get_order_status_list;
        pub mod get_order_transaction_details;
        pub mod get_orders;
        pub mod get_orders_by_email;
        pub mod get_orders_by_phone;
        pub mod get_receipt;
        pub mod get_series;
        pub mod set_order_fields;
        pub mod set_order_payment;
        pub mod set_order_product_fields;
        pub mod set_order_receipt;
        pub mod set_order_status;
    }

    pub mod product_catalog {
        pub mod add_inventory;
        pub mod add_inventory_category;
        pub mod add_inventory_manufacturer;
        pub mod add_inventory_price_group;
        pub mod add_inventory_product;
        pub mod add_inventory_warehouse;
        pub mod delete_inventory;
        pub mod delete_inventory_category;
        pub mod delete_inventory_manufacturer;
        pub mod delete_inventory_price_group;
        pub mod delete_inventory_product;
        pub mod delete_inventory_warehouse;
        pub mod get_inventories;
        pub mod get_inventory_available_text_field_keys;
        pub mod get_inventory_categories;
        pub mod get_inventory_extra_fields;
        pub mod get_inventory_integrations;
        pub mod get_inventory_manufacturers;
        pub mod get_inventory_price_groups;
        pub mod get_inventory_product_logs;
        pub mod get_inventory_products_data;
        pub mod get_inventory_products_list;
        pub mod get_inventory_products_prices;
        pub mod get_inventory_products_stock;
        pub mod get_inventory_warehouses;
        pub mod update_inventory_products_prices;
        pub mod update_inventory_products_stock;
    }
}

pub mod baselinker;
pub mod common;
