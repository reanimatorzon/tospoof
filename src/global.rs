//! Contains types, consts and functions used throughout modules

// pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type Result<T> = anyhow::Result<T>;

#[cfg(target_os = "windows")]
pub const HOSTS_FILE_LOCATION: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

#[cfg(target_family = "unix")]
pub const HOSTS_FILE_LOCATION: &str = "/etc/hosts";

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_whitespace() {
        assert_eq!(
            "abcdef",
            remove_whitespace(" a \t\t b \t c \n d \r e \r\n f \n")
        );
    }

    #[test]
    fn test_parse_function_args_no_args() {
        assert_eq!(
            ("func_no_args".to_string(), vec![]),
            parse_function_args(" \r\n func_no_args \r\n ")
        );
    }

    #[test]
    fn test_parse_function_args_three_args() {
        assert_eq!(
            (
                "func(3)".to_string(),
                vec!["one".to_string(), "two".to_string(), "three".to_string()]
            ),
            parse_function_args(" \r\n func(one, two, three) \r\n ")
        );
    }
}
