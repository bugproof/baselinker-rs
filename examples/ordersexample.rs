use std::borrow::Borrow;
use std::collections::HashMap;
use chrono::{Utc};
use rust_decimal_macros::dec;
use baselinker::baselinker::BaseLinkerClient;
use baselinker::requests::orders::add_order::{AddOrder, Product};
use baselinker::requests::orders::get_order_status_list::GetOrderStatusList;
use baselinker::requests::product_catalog::add_inventory_product::AddInventoryProduct;
use baselinker::requests::product_catalog::get_inventories::GetInventories;
use baselinker::requests::product_catalog::get_inventory_products_data::GetInventoryProductsData;
use baselinker::requests::product_catalog::get_inventory_products_list::GetInventoryProductsList;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("BASELINKER_TOKEN").expect("BASELINKER_TOKEN environment variable");
    let mut baselinker = BaseLinkerClient::new(token, reqwest::Client::new());

    let inventories = baselinker.send(&GetInventories {}).await?.inventories;

    println!("{:?}", inventories);

    let default_inventory = &inventories[0];

    // Add new product
    let mut product_text_fields = HashMap::new();
    product_text_fields.insert("name".to_string(), "Watermelon".to_string());
    product_text_fields.insert("description".to_string(), "A juicy watermelon".to_string());

    let new_product = AddInventoryProduct {
        inventory_id: default_inventory.inventory_id,
        product_id: None,
        parent_id: None,
        is_bundle: false,
        bundle_products: None,
        ean: "".to_string(),
        sku: "".to_string(),
        tax_rate: dec!(23),
        weight: 4.0,
        height: 0.0,
        width: 0.0,
        length: 0.0,
        star: 0,
        manufacturer_id: 0,
        category_id: 0,
        prices: Default::default(),
        stock: Default::default(),
        locations: Default::default(),
        text_fields: product_text_fields,
        images: vec!["url:https://i.imgur.com/CBZZ5EC.png".to_string()],
        links: Default::default(),
    };

    let add_product_response = baselinker.send(&new_product).await?;
    let added_product_id = add_product_response.product_id;

    println!("Product added. Id: {}", added_product_id);

    let order_statuses = baselinker.send(&GetOrderStatusList {}).await?.statuses;
    println!("{:?}", order_statuses);

    /*let products_list = baselinker.send(
        &GetInventoryProductsList {
            inventory_id: 0,
            filter_id: Some(added_product_id),
            filter_category_id: None,
            filter_ean: None,
            filter_sku: None,
            filter_name: None,
            filter_price_from: None,
            filter_price_to: None,
            filter_stock_from: None,
            filter_stock_to: None,
            page: None,
            filter_sort: None
        }).await?.products;

    let product = products.values().next().unwrap();*/

    let added_order_id = baselinker.send(&AddOrder {
        order_status_id: 222146,
        custom_source_id: None,
        date_add: Utc::now(),
        user_comments: "".to_string(),
        admin_comments: "".to_string(),
        phone: "".to_string(),
        email: "".to_string(),
        user_login: "".to_string(),
        currency: "".to_string(),
        payment_method: "".to_string(),
        payment_method_cod: "".to_string(),
        paid: true,
        delivery_method: "".to_string(),
        delivery_price: Default::default(),
        delivery_fullname: "".to_string(),
        delivery_company: "".to_string(),
        delivery_address: "".to_string(),
        delivery_city: "".to_string(),
        delivery_postcode: "".to_string(),
        delivery_country_code: "".to_string(),
        delivery_point_id: "".to_string(),
        delivery_point_name: "".to_string(),
        delivery_point_address: "".to_string(),
        delivery_point_postcode: "".to_string(),
        delivery_point_city: "".to_string(),
        invoice_fullname: "".to_string(),
        invoice_company: "".to_string(),
        invoice_nip: "".to_string(),
        invoice_address: "".to_string(),
        invoice_city: "".to_string(),
        invoice_postcode: "".to_string(),
        invoice_country_code: "".to_string(),
        want_invoice: false,
        custom_extra_fields: Default::default(),
        products: vec![Product {
            storage: "".to_string(),
            storage_id: 0,
            product_id: added_product_id.to_string(),
            variant_id: 0,
            name: new_product.name.clone(),
            sku: new_product.sku.clone(),
            ean: new_product.ean.clone(),
            location: "".to_string(),
            warehouse_id: 0,
            price_brutto: Default::default(),
            tax_rate: Default::default(),
            quantity: 5,
            weight: 0.0,
        }],
    }).await?.order_id;

    println!("Order added. Id: {}", added_order_id);

    Ok(())
}
