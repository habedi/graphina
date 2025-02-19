use std::ffi::OsString;
use tracing::error;

pub fn run(args: impl IntoIterator<Item = OsString>) -> Result<(), i32> {
    let _args: Vec<OsString> = args.into_iter().collect();
    // Your implementation here
    // Expecting at least 2 arguments
    if _args.len() < 2 {
        error!("Expecting at least 2 arguments");
        return Err(1);
    }
    Ok(())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn test_run_with_valid_args() {
        let args = vec![OsString::from("arg1"), OsString::from("arg2")];
        let result = run(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_invalid_args() {
        let args = vec![OsString::from("invalid_arg")];
        let result = run(args);
        assert!(result.is_err());
    }
}
