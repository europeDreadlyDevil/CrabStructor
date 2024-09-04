

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crabstructor::derive::Constructor;
    #[test]
    fn lib_test_int() {

        #[derive(Constructor, PartialEq, Debug)]
        struct Example {
            #[init(10)]
            field: i32,
        }

        assert_eq!(Example::new(), Example {field: 10})
    }

    #[test]
    fn lib_test_string() {

        #[derive(Constructor, PartialEq, Debug)]
        struct Example {
            #[init("my_string")]
            field: String,
        }

        assert_eq!(Example::new(), Example {field: "my_string".to_string()})
    }

    #[test]
    fn lib_test_str_ref() {

        #[derive(Constructor, PartialEq, Debug)]
        struct Example<'a> {
            #[init("my_string")]
            field: &'a str,
        }

        assert_eq!(Example::new(), Example {field: "my_string"})
    }

    #[test]
    fn lib_test_bool() {
        #[derive(Constructor, PartialEq, Debug)]
        struct Example {
            #[init(true)]
            field: bool,
        }

        assert_eq!(Example::new(), Example {field: true})
    }
    
    #[test]
    fn lib_test_default() {
        #[derive(Constructor, PartialEq, Debug)]
        struct Example {
            #[init(default)]
            field: bool,
        }

        assert_eq!(Example::new(), Example {field: bool::default()})
    }
    
    #[test]
    fn lib_test_f64() {
        #[derive(Constructor, PartialEq, Debug)]
        struct Example {
            #[init(2.0)]
            field: f64,
        }

        assert_eq!(Example::new(), Example {field: 2.0})
    }
    
    #[test]
    fn lib_test_new_arc() {
        #[derive(Constructor, PartialEq, Debug)]
        struct Example {
            #[new("string")]
            field: Arc<String>,
        }

        assert_eq!(Example::new(), Example {field: Arc::new("string".into())})
    }

    #[test]
    fn lib_test_new_arc_with_var() {
        #[derive(Constructor, PartialEq, Debug)]
        struct Example {
            #[new(arc_string: String)]
            field: Arc<String>,
        }

        assert_eq!(Example::new("string".to_string()), Example {field: Arc::new("string".into())})
    }
}

fn main() {
    
}