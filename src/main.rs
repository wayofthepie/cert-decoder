use std::path::Path;

trait PathValidator {
    fn is_file(&self, path: &str) -> bool;
}

struct CertValidator;

impl PathValidator for CertValidator {
    fn is_file(&self, path: &str) -> bool {
        Path::new(path).is_file()
    }
}

fn execute(validator: impl PathValidator, args: Vec<String>) -> Result<(), String> {
    if args.len() != 1 {
        let error = format!(
            "{}{}",
            "Error: did not receive a single argument, ",
            "please invoke cert-decoder as follows: ./cert-decoder /path/to/cert."
        );
        return Err(error);
    }
    let path = &args[0];
    if !validator.is_file(path) {
        return Err(
            "Error: path given is not a regular file, please update to point to a certificate."
                .to_owned(),
        );
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let args = std::env::args().skip(1).collect();
    let validator = CertValidator;
    execute(validator, args)
}

#[cfg(test)]
mod test {

    use crate::{execute, PathValidator};

    struct FakeValidator {
        is_file: bool,
    }

    impl PathValidator for FakeValidator {
        fn is_file(&self, _: &str) -> bool {
            self.is_file
        }
    }

    #[test]
    fn should_error_if_not_given_a_single_argument() {
        // arrange
        let args = Vec::new();
        let validator = FakeValidator { is_file: false };

        // act
        let result = execute(validator, args);

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
    fn should_error_if_argument_is_not_a_regular_file() {
        // arrange
        let args = vec!["not-a-regular-file".to_owned()];
        let validator = FakeValidator { is_file: false };

        // act
        let result = execute(validator, args);

        // assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Error: path given is not a regular file, please update to point to a certificate."
        );
    }
}
