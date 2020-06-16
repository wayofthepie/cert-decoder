use std::path::Path;
use x509_parser::{parse_x509_der, pem::pem_to_der};

trait FileProcessor {
    fn is_file(&self, path: &str) -> bool;
    fn read_to_string(&self, path: &str) -> Result<String, Box<dyn std::error::Error>>;
}

struct CertProcessor;

impl FileProcessor for CertProcessor {
    fn is_file(&self, path: &str) -> bool {
        Path::new(path).is_file()
    }
    fn read_to_string(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(std::fs::read_to_string(path)?)
    }
}

fn execute(
    processor: impl FileProcessor,
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
    if !processor.is_file(path) {
        return Err(
            "Error: path given is not a regular file, please update to point to a certificate."
                .into(),
        );
    }
    let cert = processor.read_to_string(path)?;
    let (_, pem) = pem_to_der(cert.as_bytes())?;
    let (_, cert) = parse_x509_der(&pem.contents)?;
    let output = format!("{:#?}", cert.tbs_certificate);
    println!("{}", output);
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

    #[derive(Default)]
    struct FakeProcessor {
        is_file: bool,
        file_str: String,
    }

    impl FileProcessor for FakeProcessor {
        fn is_file(&self, _: &str) -> bool {
            self.is_file
        }
        fn read_to_string(&self, _: &str) -> Result<String, Box<dyn std::error::Error>> {
            Ok(self.file_str.clone())
        }
    }

    #[test]
    fn should_error_if_not_given_a_single_argument() {
        let args = Vec::new();
        let validator = FakeProcessor::default();
        let result = execute(validator, args);
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
        let args = vec!["not-a-regular-file".to_owned()];
        let validator = FakeProcessor::default();
        let result = execute(validator, args);
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.err().unwrap()),
            "Error: path given is not a regular file, please update to point to a certificate."
        );
    }

    #[test]
    fn should_error_if_given_argument_is_not_a_pem_encoded_certificate() {
        let args = vec!["Cargo.toml".to_owned()];
        let validator = FakeProcessor {
            is_file: true,
            ..FakeProcessor::default()
        };
        let result = execute(validator, args);
        assert!(result.is_err())
    }

    #[test]
    fn should_error_if_argument_is_not_a_valid_certificate() {
        let cert = include_str!("../resources/bad.crt");
        let args = vec!["doesnt-really-matter".to_owned()];
        let processor = FakeProcessor {
            is_file: true,
            file_str: cert.to_owned(),
        };
        let result = execute(processor, args);
        assert!(result.is_err());
    }

    #[test]
    fn should_succeed() {
        let cert = include_str!("../resources/google.com.crt");
        let args = vec!["doesnt-really-matter".to_owned()];
        let validator = FakeProcessor {
            is_file: true,
            file_str: cert.to_owned(),
        };
        let result = execute(validator, args);
        println!("{:#?}", result);
        assert!(result.is_ok());
    }
}
