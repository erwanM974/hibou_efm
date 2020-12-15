/********************************************************************************
 * Copyright (c) 2020 Erwan Mahe (github.com/erwanM974)
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
 * which is available at https://www.apache.org/licenses/LICENSE-2.0.
 *
 * SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
 ********************************************************************************/

use std::collections::BTreeMap;
use std::collections::{HashSet,HashMap};

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::*;
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::make_frontier;

use crate::core::syntax::data::builtin::integer::TD_Integer;
use crate::core::syntax::data::builtin::bool::TD_Bool;
use crate::core::syntax::data::builtin::float::TD_Float;
use crate::core::syntax::data::generic::TD_Generic;

use crate::process::hibou_process::*;

use crate::xlia::model::generate_xlia_model;

use crate::grpc_connect::calls::*;

use crate::diversity::symbex_client::SymbexClient;
use crate::diversity::*;

use crate::core::semantics::shape_execute::shape_execute;

use crate::grpc_connect::to_grpc::{td_generic_to_grpc,td_bool_to_grpc};

use crate::grpc_connect::xlia_reference_name_tools::*;
use crate::process::deploy_receptions::deploy_original_action_followup;




pub enum TraceSymbexResult {
    Sat(u32,TD_Bool), //new_ec_id,trace_firing_condition
    UnSat(TD_Bool) // trace_firing_condition
}

pub async fn trace_symbolic_execution(  client : &mut SymbexClient<tonic::transport::Channel>,
                                        gen_ctx : &GeneralContext,
                                        exe_ctx : &mut ExecutionContext,
                                        lf_id : usize,
                                        ms_id : usize,
                                        trace_params : &Vec<TD_Generic>,
                                        trace_delay_opt : &Option<TD_Float>,
                                        temporality : &HibouProcessTemporality,
                                        parent_diversity_ec_id : u32)
                                        -> TraceSymbexResult {
    // ***
    let mut param_counter : usize = 0;
    let mut variable_diversity_values : Vec<VariableValuePair> = Vec::new();
    for trace_param_td_gen in trace_params {
        let trace_param_diversity_fqn = trace_message_parameter_diversity_fqn(gen_ctx,exe_ctx,lf_id,ms_id,param_counter);
        variable_diversity_values.push( VariableValuePair{variable_id:trace_param_diversity_fqn,
            value : Some(td_generic_to_grpc(gen_ctx,exe_ctx,lf_id,trace_param_td_gen))} );
        param_counter = param_counter +1;
    }
    // ***
    match temporality {
        HibouProcessTemporality::UnTimed => {
            // nothing
        },
        HibouProcessTemporality::Timed => {
            match trace_delay_opt {
                None => {
                    panic!();
                },
                Some( delay_td_float ) => {
                    let trace_delay_fqn = trace_delay_diversity_fqn(gen_ctx,exe_ctx,lf_id);
                    variable_diversity_values.push( VariableValuePair{variable_id:trace_delay_fqn,
                        value : Some(td_generic_to_grpc(gen_ctx,exe_ctx,lf_id,&TD_Generic::Float(delay_td_float.clone())))} );
                }
            }
        }
    }
    // ***
    let target_action_fqn = trace_action_compare_diversity_fqn(gen_ctx,exe_ctx,lf_id,ms_id);
    match symbex_fire_action(gen_ctx,
                             exe_ctx,
                             parent_diversity_ec_id,
                             client,
                             target_action_fqn,
                             variable_diversity_values).await {
        SymbexResult::UnSAT => {
            return TraceSymbexResult::UnSat( TD_Bool::FALSE );
        },
        SymbexResult::Success( symbex_result ) => {
            return TraceSymbexResult::Sat( symbex_result.new_diversity_ec_id,
                                           symbex_result.firing_condition);
        }
    }
}

