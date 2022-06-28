use std::collections::HashMap;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use baselinker::baselinker::BaseLinkerClient;
use baselinker::common::Error;
use baselinker::requests::orders::add_order::*;
use baselinker::requests::product_catalog::*;
use baselinker::requests::product_catalog::add_inventory::AddInventory;
use baselinker::requests::product_catalog::add_inventory_product::AddInventoryProduct;
use baselinker::requests::product_catalog::get_inventories::GetInventories;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("BASELINKER_TOKEN").expect("BASELINKER_TOKEN environment variable");
    let mut baselinker = BaseLinkerClient::new(token, reqwest::Client::new());

    let inventories = baselinker.send(&GetInventories {}).await?.inventories;

    println!("{:?}", inventories);

    let default_inventory = &inventories[0];

    // Add new product
    let mut product_text_fields = HashMap::<String, String>::new();
    product_text_fields.insert("name".to_string(), "Watermelon".to_string());
    product_text_fields.insert("description".to_string(), "A juicy watermelon".to_string());

    // TODO: add more examples
    let add_product_response = baselinker.send(&AddInventoryProduct {
        inventory_id: default_inventory.inventory_id.to_string(),
        product_id: None,
        parent_id: None,
        is_bundle: false,
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
        images: vec![],
        links: Default::default(),
        bundle_products: None
    }).await?;

    println!("Product created. Id: {}", add_product_response.product_id);

    // TODO: Add new order

    Ok(())
}
