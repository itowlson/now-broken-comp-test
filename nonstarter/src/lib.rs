cargo_component_bindings::generate!();

struct Component;

impl crate::bindings::exports::fermyon::spin::inbound_http::Guest for Component {
    fn handle_request(_req: crate::bindings::fermyon::spin::http_types::Request,) -> crate::bindings::fermyon::spin::http_types::Response {
        println!("HELLO THIS IS NONSTARTER HANDLING");
        crate::bindings::fermyon::spin::http_types::Response {
            status: 200,
            headers: None,
            body: None,
        }
    }
}
