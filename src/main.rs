fn execute(args: Vec<String>) -> Result<(), ()> {
    if args.len() != 1 {
        return Err(());
    }
    Ok(())
}

fn main() -> Result<(), ()> {
    let args = std::env::args().skip(1).collect();
    execute(args)
}

#[cfg(test)]
mod test {
    use crate::execute;

    #[test]
    fn should_error_if_no_arguments_are_given() {
        let args = Vec::new();
        let result = execute(args);
        assert!(result.is_err());
    }
}
