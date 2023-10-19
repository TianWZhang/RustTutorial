fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ls() {
        let mut cmd = std::process::Command::new("ls");
        let res = cmd.output();
        assert!(res.is_ok());
    }
// echo $PATH | tr : '\n' # tr (translate characters) to replace the colons (:) with the newlines(\n)

    #[test]
    fn test_hello() {
        let mut cmd = assert_cmd::Command::cargo_bin("l1_hello").unwrap();
        cmd.assert().success().stdout("Hello, world!\n");
    }

    #[test]
    fn true_ok() {
        let mut cmd = assert_cmd::Command::cargo_bin("true").unwrap();
        cmd.assert().success();
    }

    #[test]
    fn false_not_ok() {
        let mut cmd = assert_cmd::Command::cargo_bin("false").unwrap();
        cmd.assert().failure();
    }
}