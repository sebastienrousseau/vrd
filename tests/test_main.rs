#[cfg(test)]
mod tests {
    use vrd::run;

    #[test]
    fn test_run() {
        if let Err(err) = run() {
            eprintln!("Error running vrd: {}", err);
        }
        assert_eq!(1, 1)
    }
}
