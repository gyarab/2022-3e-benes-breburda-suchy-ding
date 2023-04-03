pub fn resp(status: u16, body: &impl serde::Serialize) -> tide::Result {
    let body = tide::Body::from_json(body)?;
    Ok(tide::Response::builder(status).body(body).build())
}
