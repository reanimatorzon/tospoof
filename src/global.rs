use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn parse_function_args(sign: &str) -> (String, Vec<String>) {
    let sign = remove_whitespace(sign);
    if let Some(idx) = sign.find('(') {
        let args = &sign[idx + 1..sign.len() - 1];
        let args: Vec<String> = args.split(',').map(String::from).collect();
        let args: Vec<String> = args.into_iter().filter(|arg| !arg.is_empty()).collect();

        let sign = &sign[..idx];

        let parentheses = if !args.is_empty() {
            format!("({})", args.len())
        } else {
            "".to_string()
        };

        (format!("{}{}", sign, parentheses), args)
    } else {
        (sign, vec![])
    }
}

pub fn remove_whitespace(str: &str) -> String {
    str.chars().filter(|c| !c.is_whitespace()).collect()
}
