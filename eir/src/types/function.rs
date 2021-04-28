use super::type_::Type;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Function {
    argument: Arc<Type>,
    result: Arc<Type>,
}

impl Function {
    pub fn new(argument: impl Into<Type>, result: impl Into<Type>) -> Self {
        Self {
            argument: argument.into().into(),
            result: result.into().into(),
        }
    }

    pub fn argument(&self) -> &Type {
        &self.argument
    }

    pub fn arguments(&self) -> impl IntoIterator<Item = &Type> {
        let mut arguments = vec![self.argument()];
        let mut type_ = self;

        while let Type::Function(function) = type_.result() {
            arguments.push(function.argument());
            type_ = function;
        }

        arguments
    }

    pub fn result(&self) -> &Type {
        &self.result
    }

    pub fn last_result(&self) -> &Type {
        let mut type_ = self;

        while let Type::Function(function) = type_.result() {
            type_ = function;
        }

        type_.result()
    }
}

#[cfg(test)]
mod tests {
    use super::super::primitive::Primitive;
    use super::*;

    #[test]
    fn argument() {
        assert_eq!(
            Function::new(Primitive::Number, Primitive::Number).argument(),
            &Primitive::Number.into()
        );
    }

    #[test]
    fn result() {
        assert_eq!(
            Function::new(Primitive::Number, Primitive::Number).result(),
            &Primitive::Number.into()
        );
    }

    #[test]
    fn arguments() {
        assert_eq!(
            Function::new(Primitive::Number, Primitive::Number,)
                .arguments()
                .into_iter()
                .collect::<Vec<&Type>>(),
            vec![&Primitive::Number.into()]
        );

        assert_eq!(
            Function::new(
                Primitive::Number,
                Function::new(Primitive::Number, Primitive::Number)
            )
            .arguments()
            .into_iter()
            .collect::<Vec<&Type>>(),
            vec![&Primitive::Number.into(), &Primitive::Number.into()]
        );
    }

    #[test]
    fn last_result() {
        assert_eq!(
            Function::new(
                Primitive::Number,
                Function::new(Primitive::Number, Primitive::Number)
            )
            .last_result(),
            &Primitive::Number.into()
        );
    }
}
