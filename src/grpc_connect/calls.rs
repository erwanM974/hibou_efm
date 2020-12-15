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


use crate::diversity::symbex_client::SymbexClient;
use crate::diversity::model_definition_request::ModelAlt;
use crate::diversity::*;

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;

use crate::core::syntax::action::*;
use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::builtin::bool::TD_Bool;
use crate::grpc_connect::to_grpc::{td_generic_to_grpc,td_bool_to_grpc};
use crate::grpc_connect::from_grpc::expression_from_grpc;
use crate::grpc_connect::xlia_reference_name_tools::{action_diversity_fqn,variable_diversity_fqn,open_scopes_action_diversity_fqn};




pub async fn symbex_open_scopes(client : &mut SymbexClient<tonic::transport::Channel>, gen_ctx : &GeneralContext, exe_ctx : &mut ExecutionContext, initial_diversity_ec_id : u32) -> u32 {
    let mut current_diversity_ec_id : u32 = initial_diversity_ec_id;
    for lf_id in 0..gen_ctx.get_lf_num() {
        let open_scopes_action_fqn = open_scopes_action_diversity_fqn(gen_ctx,lf_id);
        let eval_machine_request = SymbexEvalRunnableRequest {
            execution_context_id: current_diversity_ec_id,
            runnable_element_id: open_scopes_action_fqn,
            variable_value: Vec::new()
        };
        println!("EVAL MACHINE REQUEST for OPEN SCOPE on lf {} = {:?}",lf_id, eval_machine_request);
        let eval_machine_response = client.symbex_eval_basic_machine(tonic::Request::new(eval_machine_request)).await.unwrap();
        let eval_machine_reply : SymbexEvalRunnableBasicReply = eval_machine_response.into_inner();
        println!("EVAL MACHINE RESPONSE = {:?}", eval_machine_reply);
        current_diversity_ec_id = eval_machine_reply.execution_context_id;
        // ***
        for symbol_creation_report in &eval_machine_reply.created_symbols {
            let symbol_fqn = &symbol_creation_report.symbol_id;
            let symbol_type = TD_DataType::from_grpc(symbol_creation_report.r#type);
            println!("DIVERSITY created the symbol '{:?}' of type '{:?}'...", symbol_fqn, symbol_type);
            let new_symbol_id = exe_ctx.add_diversity_symbol(symbol_fqn,&symbol_type);
            println!("...kept track of in HIBOU context as #{}", new_symbol_id);
        }
    }
    return current_diversity_ec_id;
}


pub enum SymbexResult {
    Success(SymbexResultSuccess),
    UnSAT
}

pub struct SymbexResultSuccess {
    pub new_diversity_ec_id : u32,
    pub firing_condition : TD_Bool
}

/**
- action_name : &String
  name of the action to fire in the xlia model
  it corresponds to the position of the parent action within the initial model
- ec_id : u32
  identifier of the execution context
- variables_to_update : Vec<(String,TD_Generic)>
  list of variables to update in the xlia model
   + first arg is the full identifier of the variable
   + second arg is the term which aims to replace it
**/
pub async fn symbex_fire_action(    gen_ctx : &GeneralContext,
                                    exe_ctx : &mut ExecutionContext,
                                    diversity_ec_id : u32,
                                    client : &mut SymbexClient<tonic::transport::Channel>,
                                    target_action_fqn : String,
                                    variables_to_update : Vec<VariableValuePair>)
                -> SymbexResult {

    let eval_machine_request = SymbexEvalRunnableRequest {
        execution_context_id: diversity_ec_id.into(),
        runnable_element_id: target_action_fqn.into(),
        variable_value: variables_to_update
    };
    println!("EVAL MACHINE REQUEST = {:?}", eval_machine_request);

    let eval_machine_response = client.symbex_eval_basic_machine(tonic::Request::new(eval_machine_request)).await.unwrap();
    let eval_machine_reply : SymbexEvalRunnableBasicReply = eval_machine_response.into_inner();
    println!("EVAL MACHINE REPLY = {:?}", eval_machine_reply);

    if eval_machine_reply.is_satisfiable {
        // ***
        let new_diversity_ec_id : u32 = eval_machine_reply.execution_context_id;
        // ***
        for symbol_creation_report in eval_machine_reply.created_symbols {
            let symbol_fqn : String = symbol_creation_report.symbol_id;
            let symbol_type = TD_DataType::from_grpc(symbol_creation_report.r#type);
            println!("DIVERSITY created the symbol '{:?}' of type '{:?}'...", symbol_fqn, symbol_type);
            let new_symbol_id = exe_ctx.add_diversity_symbol(&symbol_fqn,&symbol_type);
            println!("...kept track of in HIBOU context as #{}", new_symbol_id);
        }
        // ***
        let path_condition_expr = eval_machine_reply.path_condition.unwrap();
        let pc_td_gen = expression_from_grpc(gen_ctx,exe_ctx,&path_condition_expr, &TD_DataType::Bool).unwrap();
        let path_condition : TD_Bool = pc_td_gen.as_td_bool();
        println!("DIVERSITY provided the path condition '{:?}'\nin the new context {:?}...", &path_condition,&new_diversity_ec_id);
        exe_ctx.set_path_condition(path_condition);
        println!("...updated in HIBOU context");
        // ***
        let firing_condition_expr = eval_machine_reply.other_condition.unwrap();
        let fc_td_gen = expression_from_grpc(gen_ctx,exe_ctx,&firing_condition_expr, &TD_DataType::Bool).unwrap();
        let firing_condition : TD_Bool = fc_td_gen.as_td_bool();
        // ***
        let symbex_success = SymbexResultSuccess{new_diversity_ec_id,firing_condition};
        return SymbexResult::Success(symbex_success);
    } else {
        return SymbexResult::UnSAT;
    }
}


pub async fn symbex_request_variable(   gen_ctx : &GeneralContext,
                                        exe_ctx : &ExecutionContext,
                                        diversity_ec_id : u32,
                                        client : &mut SymbexClient<tonic::transport::Channel>,
                                        var_fqn : String,
                                        var_expected_type : &TD_DataType) -> TD_Generic {

    // ***
    let query_value_request = tonic::Request::new(QueryValueForVariableRequest {
        execution_context_id: diversity_ec_id.into(),
        variable_id:vec![var_fqn]
    });
    println!("QUERY VALUE REQUEST = {:?}", query_value_request);

    let query_value_response = client.query_valueof_variable(query_value_request).await.unwrap();
    let query_value_reply : QueryValueForVariableReply = query_value_response.into_inner();
    println!("QUERY VALUE REPLY = {:?}", query_value_reply);

    let var_value_pair : &VariableValuePair = query_value_reply.variable_value.get(0).unwrap();
    match &var_value_pair.value {
        None => {
            panic!();
        },
        Some(ref grpc_expression) => {
            return expression_from_grpc(gen_ctx,exe_ctx,&grpc_expression,var_expected_type).unwrap();
        }
    }
}
