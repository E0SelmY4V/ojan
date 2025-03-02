pub struct FloatType {}
pub struct IntegerType {}
pub trait TypeCategory: private::Sealed {}
impl TypeCategory for FloatType {}
impl TypeCategory for IntegerType {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::FloatType {}
    impl Sealed for super::IntegerType {}
}

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
}

pub trait ValueCategory {
    type Category: TypeCategory;
}

impl ValueCategory for i8 {
    type Category = IntegerType;
}
impl ValueCategory for i16 {
    type Category = IntegerType;
}
impl ValueCategory for i32 {
    type Category = IntegerType;
}
impl ValueCategory for f32 {
    type Category = FloatType;
}
impl ValueCategory for f64 {
    type Category = FloatType;
}


pub trait ValueT {
    fn value(&self) -> Value;
}

impl<T: ValueCategory + ValueTypeHelper<T::Category>> ValueT for T {
    fn value(&self) -> Value {
        return ValueTypeHelper::value_impl(self)
    }
}

trait ValueTypeHelper<Category: TypeCategory> {
    fn value_impl(&self) -> Value;
}

impl<T> ValueTypeHelper<IntegerType> for T
where
    T: ValueCategory<Category = IntegerType> + Into<i64> + Clone
{
    fn value_impl(&self) {
        return Value::Integer(self.clone().into())
    }
}

impl<T> ValueTypeHelper<FloatType> for T
where
    T: ValueCategory<Category = FloatType> + Into<f64> + Clone
{
    fn value_impl(&self) {
        return Value::Float(self.clone().into())
    }
}

pub fn main() {
    println!("{:?}", 1i32.value());
    println!("{:?}", 1.0f64.value());
}
