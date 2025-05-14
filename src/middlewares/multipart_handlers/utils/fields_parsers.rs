pub fn text_to_vec_string(s: &str) -> actix_web::Result<Vec<String>> {
    let s = s
        .trim()
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid format: expected [values]"))?;

    let values: Vec<String> = s.split(',').map(|v| v.trim().to_string()).collect();

    Ok(values)
}

pub fn text_to_string_f64(s: &str) -> actix_web::Result<(String, u64)> {
    let s = s.trim().trim_matches(['[', ']', '(', ')']);
    let mut parts = s.split(',').map(str::trim);

    let first = parts
        .next()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing first value"))?
        .to_string();
    let second = parts
        .next()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing second value"))?
        .parse::<u64>()
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid number format"))?;

    if parts.next().is_some() {
        return Err(actix_web::error::ErrorBadRequest(
            "Invalid input: expected exactly two values",
        ));
    }

    Ok((first, second))
}

pub fn text_to_two_vec_f64(s: &str) -> actix_web::Result<(f64, f64)> {
    let s = s.trim().trim_matches(['[', ']', '(', ')']);
    let numbers: Vec<f64> = s
        .split(',')
        .map(|v| v.trim().parse::<f64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid number format"))?;

    match numbers.as_slice() {
        [lat, lon] => Ok((*lat, *lon)),
        _ => Err(actix_web::error::ErrorBadRequest(
            "Invalid input: expected exactly two numbers",
        )),
    }
}
