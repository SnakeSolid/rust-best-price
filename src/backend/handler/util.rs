macro_rules! check_text {
    ( $x: expr, $msg: expr ) => {{
        match $x {
            Ok(value) => value,
            Err(_) => return Ok(Response::with((status::InternalServerError, $msg))),
        }
    }}
}

macro_rules! check_error {
    ( $x: expr ) => {{
        match $x {
            Ok(value) => value,
            Err(error) => return Ok(Response::with((status::InternalServerError, error.description()))),
        }
    }}
}
