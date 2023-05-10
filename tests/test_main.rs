#[cfg(test)]
mod tests {
    use std::process;
    use vrd::run;

    #[test]
    fn test_run() {
        if let Err(err) = run() {
            eprintln!("Error running vrd: {}", err);
            process::exit(1);
        }
        assert_eq!(1, 1)
    }
}
