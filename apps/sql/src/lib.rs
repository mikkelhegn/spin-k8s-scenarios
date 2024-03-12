use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::sqlite::{Connection, Value};

/// A simple Spin HTTP component.
#[http_component]
fn handle_sql(_req: Request) -> anyhow::Result<impl IntoResponse> {

    let connection = match Connection::open("sqlite") {
        Ok(c) => c,
        Err(e) => panic!("Not able to connect to sql: {:?}", e),
    };

    // Normally this would be done in a migration script
    connection.execute(
        "CREATE TABLE IF NOT EXISTS counter(id INTEGER, count INTEGER)",
        &[],
    )?;

    let rowset = connection.execute("SELECT count FROM counter WHERE id = 1", &[])?;

    let execute_params = match rowset.rows.len() {
        // If the counter row doesn't exist, let's create one
        0 => {
            connection.execute("INSERT INTO counter VALUES (1,0)", &[])?;
            [Value::Integer(1)]
        }
        // If exists let's increment
        _ => {
            let count = rowset.rows[0].get::<i64>(0).unwrap();
            [Value::Integer(count + 1)]
        }
    };

    connection.execute(
        "UPDATE counter SET count = (?) WHERE id = 1",
        execute_params.as_slice(),
    )?;

    let rowset = connection.execute("SELECT count FROM counter WHERE id = 1", &[])?;

    let value = rowset.rows[0].get::<u32>(0).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(value.to_string())
        .build())
}
