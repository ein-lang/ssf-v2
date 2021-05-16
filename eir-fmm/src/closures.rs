use super::{expressions, reference_count, types, CompileError};
use std::collections::HashMap;

pub fn compile_load_entry_pointer(
    builder: &fmm::build::InstructionBuilder,
    closure_pointer: impl Into<fmm::build::TypedExpression>,
) -> Result<fmm::build::TypedExpression, fmm::build::BuildError> {
    // Entry functions of thunks need to be loaded atomically
    // to make thunk update thread-safe.
    builder.atomic_load(builder.record_address(closure_pointer, 0)?)
}

pub fn compile_load_drop_function(
    builder: &fmm::build::InstructionBuilder,
    closure_pointer: impl Into<fmm::build::TypedExpression>,
) -> Result<fmm::build::TypedExpression, fmm::build::BuildError> {
    builder.load(builder.record_address(closure_pointer, 1)?)
}

pub fn compile_load_arity(
    builder: &fmm::build::InstructionBuilder,
    closure_pointer: impl Into<fmm::build::TypedExpression>,
) -> Result<fmm::build::TypedExpression, fmm::build::BuildError> {
    builder.load(builder.record_address(closure_pointer, 2)?)
}

pub fn compile_environment_pointer(
    builder: &fmm::build::InstructionBuilder,
    closure_pointer: impl Into<fmm::build::TypedExpression>,
) -> Result<fmm::build::TypedExpression, fmm::build::BuildError> {
    Ok(fmm::build::bit_cast(
        fmm::types::Pointer::new(types::compile_unsized_environment()),
        builder.record_address(closure_pointer, 3)?,
    )
    .into())
}

pub fn compile_closure_content(
    entry_function: impl Into<fmm::build::TypedExpression>,
    drop_function: impl Into<fmm::build::TypedExpression>,
    free_variables: Vec<fmm::build::TypedExpression>,
) -> fmm::build::TypedExpression {
    let entry_function = entry_function.into();

    fmm::build::record(vec![
        entry_function.clone(),
        drop_function.into(),
        expressions::compile_arity(types::get_arity(
            entry_function.type_().to_function().unwrap(),
        ))
        .into(),
        fmm::build::record(free_variables).into(),
    ])
    .into()
}

pub fn compile_drop_function(
    module_builder: &fmm::build::ModuleBuilder,
    definition: &eir::ir::Definition,
    types: &HashMap<String, eir::types::RecordBody>,
) -> Result<fmm::build::TypedExpression, CompileError> {
    const ARGUMENT_NAME: &str = "_closure";
    const ARGUMENT_TYPE: fmm::types::Primitive = fmm::types::Primitive::PointerInteger;

    Ok(module_builder.define_anonymous_function(
        vec![fmm::ir::Argument::new(ARGUMENT_NAME, ARGUMENT_TYPE)],
        |builder| -> Result<_, CompileError> {
            let environment = builder.load(fmm::build::bit_cast(
                fmm::types::Pointer::new(types::compile_environment(definition, types)),
                compile_environment_pointer(
                    &builder,
                    fmm::build::bit_cast(
                        fmm::types::Pointer::new(types::compile_unsized_closure(
                            definition.type_(),
                            types,
                        )),
                        fmm::build::variable(ARGUMENT_NAME, ARGUMENT_TYPE),
                    ),
                )?,
            ))?;

            for (index, free_variable) in definition.environment().iter().enumerate() {
                reference_count::drop_expression(
                    &builder,
                    &builder.deconstruct_record(environment.clone(), index)?,
                    free_variable.type_(),
                    types,
                )?;
            }

            Ok(builder.return_(fmm::build::VOID_VALUE.clone()))
        },
        fmm::build::VOID_TYPE.clone(),
        fmm::types::CallingConvention::Target,
    )?)
}
