use std::path::Path;

trait PathValidator {
    fn exists(&self, path: &str) -> bool;
}

struct CertValidator;

impl PathValidator for CertValidator {
    fn exists(&self, path: &str) -> bool {
        Path::new(path).exists()
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
    if !validator.exists(path) {
        return Err(
            "Error: path given as argument does not exist, it must be a path to a certificate!"
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
        is_path: bool,
    }

    impl PathValidator for FakeValidator {
        fn exists(&self, _: &str) -> bool {
            self.is_path
        }
    }

    #[test]
    fn should_error_if_not_given_a_single_argument() {
        // arrange
        let args = Vec::new();
        let validator = FakeValidator { is_path: true };

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
    fn should_error_if_argument_is_not_a_path_which_exists() {
        // arrange
        let args = vec!["does-not-exist".to_owned()];
        let validator = FakeValidator { is_path: false };

        // act
        let result = execute(validator, args);

        // assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Error: path given as argument does not exist, it must be a path to a certificate!"
        );
    }
}
