cargo_component_bindings::generate!();

struct Component;

impl crate::bindings::exports::fermyon::spin::inbound_http::Guest for Component {
    fn handle_request(_req: crate::bindings::fermyon::spin::http_types::Request,) -> crate::bindings::fermyon::spin::http_types::Response {
        println!("HELLO THIS IS STARTER HANDLING");
        let conn = match crate::bindings::fermyon::spin::sqlite::open("default") {
            Ok(conn) => conn,
            Err(e) => return error(e),
        };
        match crate::bindings::fermyon::spin::sqlite::execute(conn, "INSERT INTO test (message) VALUES ('hello!')", &[]) {
            Ok(_) => (),
            Err(e) => return error(e),
        }

        crate::bindings::fermyon::spin::http_types::Response {
            status: 200,
            headers: None,
            body: None,
        }
    }
}

fn error(e: crate::bindings::fermyon::spin::sqlite::Error) -> crate::bindings::fermyon::spin::http_types::Response {
    let s = format!("{e:?}");
    crate::bindings::fermyon::spin::http_types::Response {
        status: 500,
        headers: None,
        body: Some(s.into_bytes()),
    }
}

// impl crate::bindings::exports::fermyon::spin::app_start::Guest for Component {
//     fn handle_app_start() -> Result<(), String> {
//         println!("HELLO THIS IS STARTER STARTING");
//         let conn = crate::bindings::fermyon::spin::sqlite::open("default")
//             .map_err(|e| format!("{e:?}"))?;
//         crate::bindings::fermyon::spin::sqlite::execute(conn, "CREATE TABLE IF NOT EXISTS test (message TEXT)", &[])
//             .map_err(|e| format!("{e:?}"))?;
//         println!("YOUR DATABASE IS READY HUZZAH");
//         Ok(())
//     }
// }
