macro_rules! check_text {
    ( $x: expr, $msg: expr ) => {{
        match $x {
            Ok(value) => value,
            Err(_) => {
                warn!("Handler error: {}", $msg);

                return Ok(Response::with((status::InternalServerError)));
            }
        }
    }};
}

macro_rules! check_error {
    ( $x: expr ) => {{
        match $x {
            Ok(value) => value,
            Err(error) => {
                warn!("Handler error: {}", error.description());

                return Ok(Response::with((status::InternalServerError)));
            }
        }
    }};
}
