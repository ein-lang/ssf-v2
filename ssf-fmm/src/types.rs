pub const FUNCTION_ARGUMENT_OFFSET: usize = 1;

pub fn compile_generic_pointer() -> fmm::types::Pointer {
    fmm::types::Pointer::new(fmm::types::Primitive::Integer8)
}

pub fn get_arity(type_: &fmm::types::Function) -> usize {
    type_.arguments().len() - FUNCTION_ARGUMENT_OFFSET
}

pub fn compile(type_: &ssf::types::Type) -> fmm::types::Type {
    match type_ {
        ssf::types::Type::Function(function) => {
            fmm::types::Pointer::new(compile_unsized_closure(function)).into()
        }
        ssf::types::Type::Index(_) => unreachable!(),
        ssf::types::Type::Primitive(primitive) => compile_primitive(primitive),
        ssf::types::Type::Record(record) => compile_record(record).into(),
        ssf::types::Type::Variant => compile_variant().into(),
    }
}

pub fn compile_primitive(primitive: &ssf::types::Primitive) -> fmm::types::Type {
    match primitive {
        ssf::types::Primitive::Boolean => fmm::types::Primitive::Boolean.into(),
        ssf::types::Primitive::Float32 => fmm::types::Primitive::Float32.into(),
        ssf::types::Primitive::Float64 => fmm::types::Primitive::Float64.into(),
        ssf::types::Primitive::Integer8 => fmm::types::Primitive::Integer8.into(),
        ssf::types::Primitive::Integer32 => fmm::types::Primitive::Integer32.into(),
        ssf::types::Primitive::Integer64 => fmm::types::Primitive::Integer64.into(),
        ssf::types::Primitive::Pointer => compile_generic_pointer().into(),
    }
}

pub fn compile_variant() -> fmm::types::Record {
    fmm::types::Record::new(vec![compile_tag().into(), compile_payload().into()])
}

pub fn compile_tag() -> fmm::types::Pointer {
    // TODO Add GC functions.
    fmm::types::Pointer::new(fmm::types::Record::new(vec![
        fmm::types::Primitive::Integer64.into(),
    ]))
}

pub fn compile_payload() -> fmm::types::Primitive {
    fmm::types::Primitive::Integer64
}

pub fn compile_record(record: &ssf::types::Record) -> fmm::types::Type {
    if record.is_boxed() {
        fmm::types::Pointer::new(fmm::types::Record::new(vec![])).into()
    } else {
        fmm::types::Record::new(record.elements().iter().map(compile).collect()).into()
    }
}

pub fn compile_sized_closure(definition: &ssf::ir::Definition) -> fmm::types::Record {
    compile_raw_closure(
        compile_entry_function_from_definition(definition),
        compile_closure_payload(definition),
    )
}

pub fn compile_closure_payload(definition: &ssf::ir::Definition) -> fmm::types::Type {
    if definition.is_thunk() {
        fmm::types::Type::Union(fmm::types::Union::new(
            vec![compile_environment(definition).into()]
                .into_iter()
                .chain(vec![compile(definition.result_type())])
                .collect(),
        ))
    } else {
        compile_environment(definition).into()
    }
}

pub fn compile_unsized_closure(function: &ssf::types::Function) -> fmm::types::Record {
    compile_raw_closure(
        compile_entry_function(function.arguments(), function.last_result()),
        compile_unsized_environment(),
    )
}

pub fn compile_raw_closure(
    entry_function: fmm::types::Function,
    environment: impl Into<fmm::types::Type>,
) -> fmm::types::Record {
    fmm::types::Record::new(vec![
        entry_function.into(),
        compile_arity().into(),
        environment.into(),
    ])
}

pub fn compile_environment(definition: &ssf::ir::Definition) -> fmm::types::Record {
    compile_raw_environment(
        definition
            .environment()
            .iter()
            .map(|argument| compile(argument.type_())),
    )
}

pub fn compile_raw_environment(
    types: impl IntoIterator<Item = fmm::types::Type>,
) -> fmm::types::Record {
    fmm::types::Record::new(types.into_iter().collect())
}

pub fn compile_unsized_environment() -> fmm::types::Record {
    fmm::types::Record::new(vec![])
}

pub fn compile_curried_entry_function(
    function: &fmm::types::Function,
    arity: usize,
) -> fmm::types::Function {
    if arity == get_arity(function) {
        function.clone()
    } else {
        fmm::types::Function::new(
            function.arguments()[..arity + FUNCTION_ARGUMENT_OFFSET].to_vec(),
            fmm::types::Pointer::new(compile_raw_closure(
                fmm::types::Function::new(
                    function.arguments()[..FUNCTION_ARGUMENT_OFFSET]
                        .iter()
                        .chain(function.arguments()[arity + FUNCTION_ARGUMENT_OFFSET..].iter())
                        .cloned()
                        .collect::<Vec<_>>(),
                    function.result().clone(),
                    fmm::types::CallingConvention::Source,
                ),
                compile_unsized_environment(),
            )),
            fmm::types::CallingConvention::Source,
        )
    }
}

pub fn compile_entry_function_from_definition(
    definition: &ssf::ir::Definition,
) -> fmm::types::Function {
    compile_entry_function(
        definition
            .arguments()
            .iter()
            .map(|argument| argument.type_()),
        definition.result_type(),
    )
}

pub fn compile_entry_function<'a>(
    arguments: impl IntoIterator<Item = &'a ssf::types::Type>,
    result: &ssf::types::Type,
) -> fmm::types::Function {
    fmm::types::Function::new(
        vec![fmm::types::Pointer::new(compile_unsized_environment()).into()]
            .into_iter()
            .chain(arguments.into_iter().map(compile))
            .collect(),
        compile(result),
        fmm::types::CallingConvention::Source,
    )
}

pub fn compile_foreign_function(
    function: &ssf::types::Function,
    calling_convention: ssf::ir::CallingConvention,
) -> fmm::types::Function {
    fmm::types::Function::new(
        function.arguments().into_iter().map(compile).collect(),
        compile(function.last_result()),
        compile_calling_convention(calling_convention),
    )
}

fn compile_calling_convention(
    calling_convention: ssf::ir::CallingConvention,
) -> fmm::types::CallingConvention {
    match calling_convention {
        ssf::ir::CallingConvention::Source => fmm::types::CallingConvention::Source,
        ssf::ir::CallingConvention::Target => fmm::types::CallingConvention::Target,
    }
}

pub fn compile_arity() -> fmm::types::Primitive {
    fmm::types::Primitive::PointerInteger
}
