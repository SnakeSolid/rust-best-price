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

macro_rules! check_params {
    ( $request: expr, $content_type: expr ) => {{
        match $request.get_ref::<UrlEncodedQuery>() {
            Ok(params) => params,
            Err(_) => {
                let response = HandlerResponse::err(format!("Required parameters not found"));

                match serde_json::to_string(&response) {
                    Ok(body) => return Ok(Response::with(($content_type, status::Ok, body))),
                    Err(error) => {
                        warn!("Handler error: {}", error.description());

                        return Ok(Response::with((status::InternalServerError)));
                    }
                }
            }
        }
    }};
}

macro_rules! check_value {
    ( $content_type: expr, $params: expr, $name: expr ) => {{
        let values = match $params.get($name) {
            Some(values) => values,
            None => {
                let response =
                    HandlerResponse::err(format!("Required parameter {} not found", $name));

                match serde_json::to_string(&response) {
                    Ok(body) => return Ok(Response::with(($content_type, status::Ok, body))),
                    Err(error) => {
                        warn!("Handler error: {}", error.description());

                        return Ok(Response::with((status::InternalServerError)));
                    }
                }
            }
        };
        let value = match values.get(0) {
            Some(value) => value,
            None => {
                let response =
                    HandlerResponse::err(format!("Parameter {} must have single value", $name));

                match serde_json::to_string(&response) {
                    Ok(body) => return Ok(Response::with(($content_type, status::Ok, body))),
                    Err(error) => {
                        warn!("Handler error: {}", error.description());

                        return Ok(Response::with((status::InternalServerError)));
                    }
                }
            }
        };

        match value.parse() {
            Ok(result) => result,
            Err(_) => {
                let response = HandlerResponse::err(format!("Parameter {} must be integer", $name));

                match serde_json::to_string(&response) {
                    Ok(body) => return Ok(Response::with(($content_type, status::Ok, body))),
                    Err(error) => {
                        warn!("Handler error: {}", error.description());

                        return Ok(Response::with((status::InternalServerError)));
                    }
                }
            }
        }
    }};
}
