use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler};
use http::{httprequest, httprequest::HttpRequest};
use std::net::TcpStream;

use super::handler::WebServiceHandler;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut TcpStream) {
        match req.method {
            httprequest::Method::Get => match &&req.resource {
                httprequest::Resoure::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();

                    match route[1] {
                        "api" => {
                            let resp = WebServiceHandler::handler(&req);

                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp = StaticPageHandler::handler(&req);

                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp = PageNotFoundHandler::handler(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
