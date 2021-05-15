use super::super::types;
use std::collections::hash_map::HashMap;

pub fn get_record_clone_function_name(record: &eir::types::Record) -> String {
    format!("eir_clone_{}", record.name())
}

pub fn get_record_drop_function_name(record: &eir::types::Record) -> String {
    format!("eir_drop_{}", record.name())
}

pub fn create_record_rc_function_type(
    record: &eir::types::Record,
    types: &HashMap<String, eir::types::RecordBody>,
) -> fmm::types::Function {
    fmm::types::Function::new(
        vec![types::compile_record(record, types)],
        fmm::build::VOID_TYPE.clone(),
        fmm::types::CallingConvention::Target,
    )
}
