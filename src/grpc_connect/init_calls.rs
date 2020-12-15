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

use std::collections::HashSet;
use std::collections::btree_map::BTreeMap;

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

use crate::grpc_connect::xlia_reference_name_tools::*;
use crate::grpc_connect::calls::*;

use crate::core::syntax::data::builtin::integer::TD_Integer;

pub async fn symbex_init_model(gen_ctx : &GeneralContext, exe_ctx : &mut ExecutionContext, model_text : String) -> (SymbexClient<tonic::transport::Channel>,u32) {

    let mut client = SymbexClient::connect("http://[::1]:50051").await.unwrap();
    // INITIALIZATION
    let initialization_request = tonic::Request::new(InitializationRequest {
        session_id: "hibou_main".into(),
    });
    println!("INITIALIZATION REQUEST = {:?}", initialization_request);
    let initialization_response = client.initialization(initialization_request).await.unwrap();
    let initialization_reply = initialization_response.into_inner();
    println!("INITIALIZATION REPLY = {:?}", initialization_reply);
    // ***

    // XLIA MODEL PARSING
    let model_parse_request = tonic::Request::new(ModelDefinitionRequest {
        model_alt: Some( ModelAlt::ModelRawText(model_text) ),
        workflow_alt : None
    });
    println!("MODEL PARSE REQUEST = {:?}", model_parse_request);
    let model_parse_response = client.model_parse_text(model_parse_request).await.unwrap();
    let model_parse_reply = model_parse_response.into_inner();
    println!("MODEL PARSE REPLY = {:?}", model_parse_reply);
    // ***

    // EVAL INIT
    let eval_init_request = tonic::Request::new(SymbexEvalInitRequest {
        variable_value: Vec::new(),
    });
    println!("EVAL INIT REQUEST = {:?}", eval_init_request);
    let eval_init_response = client.symbex_eval_init(eval_init_request).await.unwrap();
    let eval_init_reply = eval_init_response.into_inner();
    println!("EVAL INIT REPLY = {:?}", eval_init_reply );
    // ***
    // open scopes called once at the beginning so that every variable vector in the DIVERSITY model has exactly one place for the original instance of the HIBOU meta-variable
    let current_diversity_ec_id = symbex_open_scopes(&mut client, gen_ctx, exe_ctx,eval_init_reply.execution_context_id).await;
    // ***
    return (client,current_diversity_ec_id);
}

pub async fn symbex_fire_lifeline_initializations(client : &mut SymbexClient<tonic::transport::Channel>,
                                                  gen_ctx : &GeneralContext,
                                                  exe_ctx : &mut ExecutionContext,
                                                  initial_div_ec_id : u32) -> u32 {
    let mut div_ec_id = initial_div_ec_id;
    for lf_id in 0..gen_ctx.get_lf_num() {
        let lf_name = gen_ctx.get_lf_name(lf_id).unwrap();
        let initialization_action_fqn = format!("{}.initialization",lf_name);
        // ***
        let mut lf_interpretation : BTreeMap<usize,TD_Generic>;
        match exe_ctx.get_lf_interpretation(lf_id) {
            None => {
                lf_interpretation = BTreeMap::new();
            },
            Some( got_lfint ) => {
                lf_interpretation = got_lfint.clone();
            }
        }
        // ***
        let appearing_variables : Vec<usize> = lf_interpretation.iter().map(|(k,v)| *k).collect();
        // ***
        let mut indexes_of_variables_in_diversity : Vec<VariableValuePair> = Vec::new();
        for vr_id in &appearing_variables {
            let (_,meta_var_idx) = exe_ctx.get_vr_parent_name_and_child_id(gen_ctx,*vr_id).unwrap();
            let varindex_fqn = varindex_diversity_fqn(gen_ctx,exe_ctx, lf_id,*vr_id);
            let td_gen = TD_Generic::Integer(TD_Integer::Value(meta_var_idx as i64));
            indexes_of_variables_in_diversity.push( VariableValuePair{variable_id:varindex_fqn,
                value : Some(td_generic_to_grpc(gen_ctx,exe_ctx,lf_id,&td_gen))} );
        }
        // ***
        println!("firing initialization for lifeline {}",lf_name);
        match symbex_fire_action(&gen_ctx,
                                 exe_ctx,
                                 div_ec_id,
                                 client,
                                 initialization_action_fqn,
                                 indexes_of_variables_in_diversity).await {
            SymbexResult::Success(success) => {
                div_ec_id = success.new_diversity_ec_id;
                // ***
                for vr_id in &appearing_variables {
                    let vr_type = exe_ctx.get_vr_type(gen_ctx,*vr_id).unwrap();
                    let var_fqn = variable_diversity_fqn(gen_ctx,exe_ctx,lf_id,*vr_id);
                    println!("HIBOU requested value of variable '{:?}' to DIVERSITY...",var_fqn);
                    let td_gen = symbex_request_variable(gen_ctx,exe_ctx,div_ec_id,client,var_fqn,&vr_type).await;
                    println!("...DIVERSITY provided value '{:?}'...",td_gen);
                    lf_interpretation.insert(*vr_id,td_gen);
                    println!("...updated in HIBOU intrepretation");
                }
                // ***
                exe_ctx.set_lf_interpretation(lf_id,lf_interpretation);
            },
            SymbexResult::UnSAT => {
                panic!("UnSAT at initialization step on lifeline {}", lf_name);
            }
        }
    }
    return div_ec_id;
}