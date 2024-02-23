pub mod macros;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rand_string = string!("das");

        let some = Some(rand_string);

        let result = to_result!(some);
    }
}

pub async fn res() -> io_result!(()) {
    Ok(())
}
