use crate::metadata::{
    check_metadata_format, check_storage_context_struct_tag, get_metadata_from_compiled_module,
    is_allowed_input_struct, is_defined_or_allowed_in_current_module,
};
use move_binary_format::binary_views::BinaryIndexedView;
use move_binary_format::errors::{Location, PartialVMError, PartialVMResult, VMError, VMResult};
use move_binary_format::file_format::{
    Bytecode, FunctionDefinition, FunctionDefinitionIndex, FunctionInstantiation, Visibility,
};
use move_binary_format::{access::ModuleAccess, CompiledModule};
use move_core_types::language_storage::ModuleId;
use move_core_types::vm_status::StatusCode;
use move_core_types::{identifier::Identifier, resolver::MoveResolver};
use move_vm_runtime::session::{LoadedFunctionInstantiation, Session};
use move_vm_types::loaded_data::runtime_types::Type;
use moveos_types::move_types::FunctionId;
use once_cell::sync::Lazy;
use std::ops::Deref;

pub static INIT_FN_NAME_IDENTIFIER: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("init").unwrap());

/// The initializer function must have the following properties in order to be executed at publication:
/// - Name init
/// - Single parameter of &mut TxContext type
/// - No return values
/// - Private
pub fn verify_init_function<S>(module: &CompiledModule, session: &Session<S>) -> VMResult<bool>
where
    S: MoveResolver,
{
    for fdef in &module.function_defs {
        let fhandle = module.function_handle_at(fdef.function);
        let fname = module.identifier_at(fhandle.name);
        if fname == INIT_FN_NAME_IDENTIFIER.as_ident_str() {
            if Visibility::Private != fdef.visibility {
                return Err(vm_error_for_init_func_checking(
                    StatusCode::INVALID_MAIN_FUNCTION_SIGNATURE,
                    "init function should private",
                    fdef,
                    module.self_id(),
                ));
            } else if fdef.is_entry {
                return Err(vm_error_for_init_func_checking(
                    StatusCode::INVALID_MAIN_FUNCTION_SIGNATURE,
                    "init function should not entry function",
                    fdef,
                    module.self_id(),
                ));
            } else {
                let function_id =
                    FunctionId::new(module.self_id(), INIT_FN_NAME_IDENTIFIER.clone());
                let loaded_function = session.load_function(
                    &module.self_id(),
                    &function_id.function_name,
                    vec![].as_slice(),
                )?;
                let parameters_usize = loaded_function.parameters.len();
                if parameters_usize != 1 && parameters_usize != 2 {
                    return Err(vm_error_for_init_func_checking(
                        StatusCode::NUMBER_OF_TYPE_ARGUMENTS_MISMATCH,
                        "init function only should have two parameter with signer or storageContext",
                        fdef,
                        module.self_id(),
                    ));
                }
                for ref ty in loaded_function.parameters {
                    match ty {
                        Type::Reference(bt) | Type::MutableReference(bt) => match bt.as_ref() {
                            Type::Struct(s) | Type::StructInstantiation(s, _) => {
                                let struct_type = session.get_struct_type(*s).unwrap();
                                if !check_storage_context_struct_tag(format!(
                                    "{}::{}::{}",
                                    struct_type.module.short_str_lossless(),
                                    struct_type.module.name(),
                                    struct_type.name
                                )) {
                                    return Err(vm_error_for_init_func_checking(
                                            StatusCode::TYPE_MISMATCH,
                                            "init function should not input structures other than storageContext",
                                            fdef,
                                            module.self_id(),
                                        ));
                                }
                            }
                            Type::Signer => {}
                            _ => {
                                return Err(vm_error_for_init_func_checking(
                                        StatusCode::TYPE_MISMATCH,
                                        "init function should only enter reference signer or mutable reference storageContext",
                                        fdef,
                                        module.self_id(),
                                    ));
                            }
                        },
                        Type::Signer => {}
                        _ => {
                            return Err(vm_error_for_init_func_checking(
                                StatusCode::TYPE_MISMATCH,
                                "init function should only enter signer or storageContext",
                                fdef,
                                module.self_id(),
                            ))
                        }
                    }
                }
            }
        }
    }
    Ok(false)
}

pub fn verify_entry_function<S>(
    func: &LoadedFunctionInstantiation,
    session: &Session<S>,
) -> PartialVMResult<bool>
where
    S: MoveResolver,
{
    if !func.return_.is_empty() {
        return Err(
            PartialVMError::new(StatusCode::INVALID_MAIN_FUNCTION_SIGNATURE)
                .with_message("function should not return values".to_string()),
        );
    }

    for ty in &func.parameters {
        if !check_transaction_input_type(ty, session) {
            return Err(PartialVMError::new(StatusCode::TYPE_MISMATCH)
                .with_message("parameter type is not allowed".to_string()));
        }
    }

    Ok(true)
}

fn check_transaction_input_type<S>(ety: &Type, session: &Session<S>) -> bool
where
    S: MoveResolver,
{
    use Type::*;
    match ety {
        // Any primitive type allowed, any parameter expected to instantiate with primitive
        Bool | U8 | U16 | U32 | U64 | U128 | U256 | Address | Signer => true,
        Vector(ety) => {
            // Vectors are allowed if element type is allowed
            check_transaction_input_type(ety.deref(), session)
        }
        Struct(idx) | StructInstantiation(idx, _) => {
            if let Some(st) = session.get_struct_type(*idx) {
                let full_name = format!("{}::{}", st.module.short_str_lossless(), st.name);
                is_allowed_input_struct(full_name)
            } else {
                false
            }
        }
        Reference(bt)
            if matches!(bt.as_ref(), Signer)
                || is_allowed_reference_types(bt.as_ref(), session) =>
        {
            // Immutable Reference to signer and specific types is allowed
            true
        }
        MutableReference(bt) if is_allowed_reference_types(bt.as_ref(), session) => {
            // Mutable references to specific types is allowed
            true
        }
        _ => {
            // Everything else is disallowed.
            false
        }
    }
}

fn is_allowed_reference_types<S>(bt: &Type, session: &Session<S>) -> bool
where
    S: MoveResolver,
{
    match bt {
        Type::Struct(sid) => {
            if let Some(st) = session.get_struct_type(*sid) {
                let full_name = format!("{}::{}", st.module.short_str_lossless(), st.name);
                if is_allowed_input_struct(full_name) {
                    return true;
                }
            }

            false
        }
        _ => false,
    }
}

fn vm_error_for_init_func_checking(
    status_code: StatusCode,
    error_message: &str,
    func_def: &FunctionDefinition,
    module_id: ModuleId,
) -> VMError {
    PartialVMError::new(status_code)
        .with_message(error_message.to_string())
        .at_code_offset(FunctionDefinitionIndex::new(func_def.function.0), 0_u16)
        .finish(Location::Module(module_id))
}

pub fn verify_private_generics(module: &CompiledModule) -> VMResult<bool> {
    if let Err(err) = check_metadata_format(module) {
        return Err(PartialVMError::new(StatusCode::MALFORMED)
            .with_message(err.to_string())
            .finish(Location::Module(module.self_id())));
    }

    let metadata_opt = get_metadata_from_compiled_module(module);
    match metadata_opt {
        None => {
            // If ROOCH_METADATA_KEY cannot be found in the metadata,
            // it means that the user's code did not use #[private_generics(T)],
            // or the user intentionally deleted the data in the metadata.
            // In either case, we will skip the verification.
            return Ok(true);
        }

        Some(metadata) => {
            let type_name_indices = metadata.private_generics_indices;
            let view = BinaryIndexedView::Module(module);

            for func in &module.function_defs {
                if let Some(code_unit) = &func.code {
                    for instr in code_unit.code.clone().into_iter() {
                        if let Bytecode::CallGeneric(finst_idx) = instr {
                            let FunctionInstantiation {
                                handle,
                                type_parameters,
                            } = view.function_instantiation_at(finst_idx);

                            let fhandle = view.function_handle_at(*handle);
                            let module_handle = view.module_handle_at(fhandle.module);

                            let module_address = view
                                .address_identifier_at(module_handle.address)
                                .to_hex_literal();
                            let module_name = view.identifier_at(module_handle.name);
                            let func_name = view.identifier_at(fhandle.name).to_string();

                            let full_path_func_name =
                                format!("{}::{}::{}", module_address, module_name, func_name);

                            let type_arguments = &view.signature_at(*type_parameters).0;
                            let private_generics_types =
                                type_name_indices.get(full_path_func_name.as_str());

                            if let Some(private_generics_types_indices) = private_generics_types {
                                for generic_type_index in private_generics_types_indices {
                                    let type_arg = type_arguments.get(*generic_type_index).unwrap();
                                    let (defined_in_current_module, struct_name) =
                                        is_defined_or_allowed_in_current_module(&view, type_arg);

                                    if !defined_in_current_module {
                                        let err_msg = format!(
                                            "resource type {:?} in function {:?} not defined in current module or not allowed",
                                            struct_name, full_path_func_name
                                        );

                                        return Err(PartialVMError::new(
                                            StatusCode::ABORT_TYPE_MISMATCH_ERROR,
                                        )
                                        .with_message(err_msg)
                                        .at_code_offset(
                                            FunctionDefinitionIndex::new(func.function.0),
                                            0_u16,
                                        )
                                        .finish(Location::Module(module.self_id())));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(true)
}
