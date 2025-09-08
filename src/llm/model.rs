
trait <LlamaModel> {

}

pub fn optimize_metadata() -> Result<String, String> {
    Err("Not yet implemented".to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn optimize_data_should_do_something() {
        let result = match super::optimize_metadata() {
            Ok(_) => todo!(),
            Err(el) => assert_eq!("Not yet implemented", el),
        };
    }
}
