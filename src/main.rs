use std::path::Path;
use x509_parser::pem::pem_to_der;

trait FileProcessor {
    fn is_file(&self, path: &str) -> bool;
}

struct CertProcessor;

impl FileProcessor for CertProcessor {
    fn is_file(&self, path: &str) -> bool {
        Path::new(path).is_file()
    }
}

fn execute(
    validator: impl FileProcessor,
    args: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() != 1 {
        let error = format!(
            "{}{}",
            "Error: did not receive a single argument, ",
            "please invoke cert-decoder as follows: ./cert-decoder /path/to/cert."
        );
        return Err(error.into());
    }
    let path = &args[0];
    if !validator.is_file(path) {
        return Err(
            "Error: path given is not a regular file, please update to point to a certificate."
                .into(),
        );
    }
    let cert = std::fs::read_to_string(path).unwrap();
    let _ = pem_to_der(cert.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect();
    let validator = CertProcessor;
    execute(validator, args)
}

#[cfg(test)]
mod test {

    use crate::{execute, FileProcessor};

    struct FakeProcessor {
        is_file: bool,
    }

    impl FileProcessor for FakeProcessor {
        fn is_file(&self, _: &str) -> bool {
            self.is_file
        }
    }

    #[test]
    fn should_error_if_not_given_a_single_argument() {
        // arrange
        let args = Vec::new();
        let validator = FakeProcessor { is_file: false };

        // act
        let result = execute(validator, args);

        // assert
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.err().unwrap()),
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
        let validator = FakeProcessor { is_file: false };

        // act
        let result = execute(validator, args);

        // assert
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.err().unwrap()),
            "Error: path given is not a regular file, please update to point to a certificate."
        );
    }

    #[test]
    fn should_error_if_given_argument_is_not_a_pem_encoded_certificate() {
        let args = vec!["Cargo.toml".to_owned()];
        let validator = FakeProcessor { is_file: true };
        let result = execute(validator, args);
        assert!(result.is_err())
    }

    #[test]
    fn should_succeed() {
        let args = vec!["resources/google.com.crt".to_owned()];
        let validator = FakeProcessor { is_file: true };
        let result = execute(validator, args);
        assert!(result.is_ok());
    }
}
