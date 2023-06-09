// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use move_binary_format::errors::{Location, PartialVMError, PartialVMResult};
use move_core_types::{gas_algebra::InternalGas, vm_status::StatusCode};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
<<<<<<< HEAD
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{StructRef, Value},
};
use smallvec::smallvec;
use std::collections::VecDeque;

use crate::natives::helpers;

// pub const MaxEmitSize: u64 = 256000;

/***************************************************************************************************
 * native fun event emit
 * Implementation of the Move native function `event::emit<T: copy + drop>(event_handle_id: &ObjectID, count: u64, data: T)`
 * Adds an event to the transaction's event log
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct EmitGasParameters {
    pub base: InternalGas,
}

impl EmitGasParameters {
=======
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};
use smallvec::smallvec;
use std::collections::VecDeque;
use std::sync::Arc;

use crate::natives::helpers;

// pub const MaxEventEmitSize: u64 = 256000;

/***************************************************************************************************
 * native fun event emit
 * Implementation of the Move native function `event::emit<T: copy + drop>(event: T)`
 * Adds an event to the transaction's event log
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct EventEmitGasParameters {
    pub base: InternalGas,
}

impl EventEmitGasParameters {
>>>>>>> ecc4197 (fixed some repeat issue)
    pub fn zeros() -> Self {
        Self {
            base: InternalGas::zero(),
        }
    }
}

<<<<<<< HEAD
pub fn native_emit(
    gas_params: &EmitGasParameters,
=======
pub fn native_write_to_event_store(
    gas_params: &EventEmitGasParameters,
>>>>>>> ecc4197 (fixed some repeat issue)
    context: &mut NativeContext,
    mut ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(args.len() == 3);

    // TODO(Gas): Charge the arg size dependent costs

    let ty = ty_args.pop().unwrap();
    let msg = args.pop_back().unwrap();
    let seq_num = pop_arg!(args, u64);
<<<<<<< HEAD
    let raw_event_handler_id = pop_arg!(args, StructRef);
    // event_handler_id equal to guid
    let event_handler_id = helpers::get_object_id(raw_event_handler_id)?;

    let _result = context
        .save_event(event_handler_id.to_bytes(), seq_num, ty, msg)
        .map_err(|e| {
            PartialVMError::new(StatusCode::ABORTED)
                .with_message(e.to_string())
                .finish(Location::Undefined)
        });
=======
    let guid = pop_arg!(args, Vec<u8>);

    // if !context.save_event(guid, seq_num, ty, msg)? {
    //     return Err(PartialVMError::new(StatusCode::ABORTED)
    //         .with_message(format!("cannot save event ")));
    // }
    let _result = context.save_event(guid, seq_num, ty, msg).map_err(|e| {
        PartialVMError::new(StatusCode::ABORTED)
            .with_message(e.to_string())
            .finish(Location::Undefined)
    });
>>>>>>> ecc4197 (fixed some repeat issue)

    Ok(NativeResult::ok(gas_params.base, smallvec![]))
}

<<<<<<< HEAD
=======
pub fn make_write_to_event_store(gas_params: EventEmitGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| {
        native_write_to_event_store(&gas_params, context, ty_args, args)
    })
}

>>>>>>> ecc4197 (fixed some repeat issue)
/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct GasParameters {
<<<<<<< HEAD
    pub emit: EmitGasParameters,
=======
    pub emit: EventEmitGasParameters,
>>>>>>> ecc4197 (fixed some repeat issue)
}

impl GasParameters {
    pub fn zeros() -> Self {
        Self {
<<<<<<< HEAD
            emit: EmitGasParameters::zeros(),
=======
            emit: EventEmitGasParameters::zeros(),
>>>>>>> ecc4197 (fixed some repeat issue)
        }
    }
}

pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
<<<<<<< HEAD
    let natives = [("emit", helpers::make_native(gas_params.emit, native_emit))];
=======
    let natives = [(
        "write_to_event_store",
        make_write_to_event_store(gas_params.emit),
    )];

>>>>>>> ecc4197 (fixed some repeat issue)
    helpers::make_module_natives(natives)
}
