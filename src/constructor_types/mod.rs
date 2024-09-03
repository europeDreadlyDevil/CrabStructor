mod lib_types;

pub trait ConstructorType {
    fn into_constructor_value(self);
}

pub struct ConstructorValue {

}
