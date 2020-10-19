use super::function::Function;
use super::value::Value;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Function(Function),
    Value(Value),
}

impl Type {
    pub fn to_id(&self) -> String {
        match self {
            Self::Function(function) => function.to_id(),
            Self::Value(value) => value.to_id(),
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self, Self::Value(Value::Primitive(_)))
    }

    pub fn into_function(self) -> Option<Function> {
        match self {
            Self::Function(function) => Some(function),
            Self::Value(_) => None,
        }
    }

    pub fn into_value(self) -> Option<Value> {
        match self {
            Self::Function(_) => None,
            Self::Value(value) => Some(value),
        }
    }
}

impl From<Function> for Type {
    fn from(function: Function) -> Self {
        Self::Function(function)
    }
}

impl<T: Into<Value>> From<T> for Type {
    fn from(value: T) -> Self {
        Self::Value(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::primitive::Primitive;
    use super::*;

    #[test]
    fn to_id() {
        assert_eq!(
            &Type::from(Function::new(
                vec![Primitive::Float64.into()],
                Primitive::Float64
            ))
            .to_id(),
            "(Float64->Float64)"
        );
    }
}
