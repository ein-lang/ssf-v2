use super::{expression::Expression, primitive_alternative::PrimitiveAlternative};
use std::{collections::HashSet, sync::Arc};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveCase {
    argument: Arc<Expression>,
    alternatives: Vec<PrimitiveAlternative>,
    default_alternative: Option<Arc<Expression>>,
}

impl PrimitiveCase {
    pub fn new(
        argument: impl Into<Expression>,
        alternatives: Vec<PrimitiveAlternative>,
        default_alternative: Option<Expression>,
    ) -> Self {
        Self {
            argument: Arc::new(argument.into()),
            alternatives,
            default_alternative: default_alternative.map(|expression| expression.into()),
        }
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn alternatives(&self) -> &[PrimitiveAlternative] {
        &self.alternatives
    }

    pub fn default_alternative(&self) -> Option<&Expression> {
        self.default_alternative
            .as_ref()
            .map(|expression| expression.as_ref())
    }

    pub(crate) fn find_variables(&self) -> HashSet<String> {
        let mut variables = self.argument.find_variables();

        for alternative in &self.alternatives {
            variables.extend(alternative.find_variables());
        }

        if let Some(default_alternative) = &self.default_alternative {
            variables.extend(default_alternative.find_variables());
        }

        variables
    }
}
