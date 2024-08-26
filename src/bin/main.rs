use crabstructor::Constructor;

#[cfg(test)]
mod test {
    use crabstructor::Constructor;
    #[test]
    fn lib_test_int() {

        #[derive(Constructor, Eq, PartialEq, Debug)]
        struct Example {
            #[init(10)]
            field: i32,
        }

        assert_eq!(Example::new(), Example {field: 10})
    }

    #[test]
    fn lib_test_string() {

        #[derive(Constructor, Eq, PartialEq, Debug)]
        struct Example {
            #[init("my_string")]
            field: String,
        }

        assert_eq!(Example::new(), Example {field: "my_string".to_string()})
    }

    #[test]
    fn lib_test_str_ref() {

        #[derive(Constructor, Eq, PartialEq, Debug)]
        struct Example<'a> {
            #[init("my_string")]
            field: &'a str,
        }

        assert_eq!(Example::new(), Example {field: "my_string"})
    }

}

fn main() {

}