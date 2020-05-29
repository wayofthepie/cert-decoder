fn execute(args: Vec<String>) -> Result<(), String> {
    if args.len() != 1 {
        return Err("Expected a path to a valid certificate!".to_owned());
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
    fn should_error_if_no_arguments_are_given() {
        // arrange
        let args = Vec::new();

        // act
        let result = execute(args);

        // assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Expected a path to a valid certificate!"
        )
    }
}
