use std::path::Path;

fn execute(args: Vec<String>) -> Result<(), String> {
    if args.len() != 1 {
        let error = format!(
            "{}{}",
            "Error: did not receive a single argument, ",
            "please invoke cert-decoder as follows: ./cert-decoder /path/to/cert."
        );
        return Err(error);
    }
    let path = Path::new(&args[0]);
    if !path.exists() {
        return Err(
            "Error: path given as argument does not exist, it must be a path to a certificate!"
                .to_owned(),
        );
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let args = std::env::args().skip(1).collect();
    execute(args)
}

#[cfg(test)]
mod test {

    use crate::execute;

    #[test]
    fn should_error_if_not_given_a_single_argument() {
        // arrange
        let args = Vec::new();

        // act
        let result = execute(args);

        // assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            format!(
                "{}{}",
                "Error: did not receive a single argument, ",
                "please invoke cert-decoder as follows: ./cert-decoder /path/to/cert."
            )
        );
    }

    #[test]
    fn should_error_if_argument_is_not_a_path_which_exists() {
        // arrange
        let args = vec!["does-not-exist".to_owned()];

        // act
        let result = execute(args);

        // assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Error: path given as argument does not exist, it must be a path to a certificate!"
        );
    }
}
