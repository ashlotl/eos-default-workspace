use std::any::Any;

pub fn convert_result_error_to_string_send<T>(
    r: Result<T, Box<dyn Any + Send>>,
) -> Result<T, String> {
    if let Err(e) = r {
        return Err(convert_error_to_string(e));
    }
    Ok(r.unwrap())
}

pub fn convert_error_to_string_send(e: Box<dyn Any + Send>) -> String {
    let maybe_string: Option<&String> = e.downcast_ref();
    match maybe_string {
        Some(v) => return v.clone(),
        None => return String::from(*e.downcast_ref::<&'static str>().unwrap()),
    }
}

pub fn convert_result_error_to_string<T>(r: Result<T, Box<dyn Any>>) -> Result<T, String> {
    if let Err(e) = r {
        return Err(convert_error_to_string(e));
    }
    Ok(r.unwrap())
}

pub fn convert_error_to_string(e: Box<dyn Any>) -> String {
    let maybe_string: Option<&String> = e.downcast_ref();
    match maybe_string {
        Some(v) => return v.clone(),
        None => return String::from(*e.downcast_ref::<&'static str>().unwrap()),
    }
}
