use crate::{
    expressions,
    types::{self, FUNCTION_ARGUMENT_OFFSET},
};
use std::collections::HashMap;

pub fn compile_foreign_declaration(
    module_builder: &fmm::build::ModuleBuilder,
    declaration: &eir::ir::ForeignDeclaration,
    types: &HashMap<String, eir::types::RecordBody>,
) -> Result<(), fmm::build::BuildError> {
    module_builder.define_variable(
        declaration.name(),
        fmm::build::record(vec![
            compile_entry_function(module_builder, declaration, types)?,
            fmm::ir::Undefined::new(types::compile_closure_drop_function()).into(),
            expressions::compile_arity(declaration.type_().arguments().into_iter().count()).into(),
            fmm::ir::Undefined::new(types::compile_unsized_environment()).into(),
        ]),
        false,
        fmm::ir::Linkage::Internal,
        None,
    );

    Ok(())
}

fn compile_entry_function(
    module_builder: &fmm::build::ModuleBuilder,
    declaration: &eir::ir::ForeignDeclaration,
    types: &HashMap<String, eir::types::RecordBody>,
) -> Result<fmm::build::TypedExpression, fmm::build::BuildError> {
    let arguments = vec![fmm::ir::Argument::new(
        "_closure",
        types::compile_untyped_closure_pointer(),
    )]
    .into_iter()
    .chain(
        declaration
            .type_()
            .arguments()
            .into_iter()
            .enumerate()
            .map(|(index, type_)| {
                fmm::ir::Argument::new(format!("arg_{}", index), types::compile(type_, types))
            }),
    )
    .collect::<Vec<_>>();

    let foreign_function_type = types::compile_foreign_function(
        declaration.type_(),
        declaration.calling_convention(),
        types,
    );

    module_builder.define_function(
        format!("{}.foreign.entry", declaration.name()),
        arguments.clone(),
        |instruction_builder| {
            Ok(instruction_builder.return_(
                instruction_builder.call(
                    module_builder.declare_function(
                        declaration.foreign_name(),
                        foreign_function_type.clone(),
                    ),
                    arguments
                        .iter()
                        .skip(FUNCTION_ARGUMENT_OFFSET)
                        .map(|argument| {
                            fmm::build::variable(argument.name(), argument.type_().clone())
                        })
                        .collect(),
                )?,
            ))
        },
        foreign_function_type.result().clone(),
        fmm::types::CallingConvention::Source,
        fmm::ir::Linkage::Internal,
    )
}
