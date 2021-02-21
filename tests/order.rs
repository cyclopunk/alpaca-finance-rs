use alpaca_finance::{ Order };
use mockito::Mock;
use std::fs::File;
use std::io::prelude::*;

mod common;

async fn base_mock(test_name: &str, mock: Mock) -> std::io::Result<Mock> {
   // Load the simulated Yahoo data we want to test against
   let mut file = File::open(format!("tests/order_data/{}.json", test_name))?;
   let mut contents = String::new();
   file.read_to_string(&mut contents)?;

   Ok(mock.with_header("content-type", "application/json")
      .with_body(&contents)
      .with_status(200))
}

#[tokio::test]
async fn get_open() {

   // GIVEN - a valid open order in place
   let alpaca = common::build_alpaca().await;
   let _m = base_mock("valid_open", common::build_mock("GET", "/v2/orders?status=open")).await.unwrap().create();

   // WHEN - we get our open orders
   let orders = Order::get_open(&alpaca).await.unwrap();

   // THEN - we get the results we expect
   assert_eq!(1, orders.len());
   assert_eq!(orders[0].id, "904837e3-3b76-47ec-b432-046db621571b");
   assert_eq!(orders[0].client_order_id, "904837e3-3b76-47ec-b432-046db621571b");
}