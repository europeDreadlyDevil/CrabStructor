

#[cfg(test)]
mod test {
    use crabstructor::Constructor;
    #[test]
    fn lib_test() {

        #[derive(Constructor, Eq, PartialEq, Debug)]
        #[constructor(
            field1 = r#"String::from("test")"#
        )]
        struct Example {
            field1: String,
            field2: i32
        }

        assert_eq!(Example::new(2), Example {field1: "test".to_string(), field2: 2})
    }
}

fn main() {}