use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs};

pub trait Handler {
    fn handler(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path).unwrap();
        let orders = serde_json::from_str(&json_contents).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handler(req: &HttpRequest) -> HttpResponse {
        let httprequest::Resoure::Path(s) = &req.resource;

        let route: Vec<&str> = s.split("/").collect();

        if route[2] == "shipping" {
            let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
            let mut headers = HashMap::new();
            headers.insert("Content-Type", "application/json");
            HttpResponse::new("200", Some(headers), body)
        } else {
            HttpResponse::new("404", None, Self::load_file("404.html"))
        }
    }
}
