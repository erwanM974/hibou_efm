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
use crate::core::syntax::data::td_type::TD_DataType;

use crate::process::hibou_process::*;

use crate::grpc_connect::calls::*;
use crate::xlia::model::generate_xlia_model;

use crate::diversity::symbex_client::SymbexClient;
use crate::diversity::*;

use crate::core::semantics::shape_execute::shape_execute;

use crate::grpc_connect::to_grpc::{td_generic_to_grpc,td_bool_to_grpc};

use crate::grpc_connect::xlia_reference_name_tools::*;
use crate::process::deploy_receptions::deploy_original_action_followup;




pub enum ModelSymbexResult {
    Sat(u32,TD_Bool,Vec<TD_Generic>,Option<TD_Float>), //new_ec_id,model_firing_condition,effective_parameters,delay
    UnSat //(HibouExecutionFailure)
}

pub async fn model_symbolic_execution(  client : &mut SymbexClient<tonic::transport::Channel>,
                                    gen_ctx : &GeneralContext,
                                    exe_ctx : &mut ExecutionContext,
                                    model_action : &ObservableAction,
                                    parent_diversity_ec_id : u32,
                                    needs_scoping : bool,
                                        temporality : &HibouProcessTemporality)
                                    -> ModelSymbexResult {
    // ***
    let appearing_variables : HashSet<usize> = gather_appearing_variables(model_action);
    // ***
    let mut variable_diversity_values : Vec<VariableValuePair> = Vec::new();
    for vr_id in &appearing_variables {
        let (_,meta_var_idx) = exe_ctx.get_vr_parent_name_and_child_id(gen_ctx,*vr_id).unwrap();
        let varindex_fqn = varindex_diversity_fqn(gen_ctx,exe_ctx, model_action.lf_act.lf_id,*vr_id);
        let td_gen = TD_Generic::Integer(TD_Integer::Value(meta_var_idx as i64));
        variable_diversity_values.push( VariableValuePair{variable_id:varindex_fqn,
            value : Some(td_generic_to_grpc(gen_ctx,exe_ctx,model_action.lf_act.lf_id,&td_gen))} );
    }
    // ***
    match model_action.act_kind {
        /*
            In case of a reception,
            the values must be provided to DIVERSITY by HIBOU
        */
        ObservableActionKind::Reception => {
            let mut pr_id : usize = 0;
            for param in &model_action.params {
                match param {
                    ValueOrNewFresh::Value(td_gen) => {
                        let param_diversity_fqn = message_parameter_diversity_fqn(gen_ctx,exe_ctx,model_action.lf_act.lf_id,model_action.ms_id,pr_id);
                        variable_diversity_values.push( VariableValuePair{variable_id:param_diversity_fqn,
                            value : Some(td_generic_to_grpc(gen_ctx,exe_ctx,model_action.lf_act.lf_id,&td_gen))} );
                    },
                    ValueOrNewFresh::NewFresh => {
                        // do nothing
                    }
                }
                pr_id = pr_id + 1;
            }
        },
        /*
            In case of an emission,
            DIVERSITY will write the new values on the emitting lifeline's message parameters variables
        */
        _ => {}
    }
    // ***
    let current_diversity_ec_id : u32;
    if needs_scoping {
        current_diversity_ec_id = symbex_open_scopes(client, gen_ctx, exe_ctx,parent_diversity_ec_id).await;
    } else {
        current_diversity_ec_id = parent_diversity_ec_id;
    }
    // ***
    let target_action_fqn = action_diversity_fqn(gen_ctx,exe_ctx,model_action.lf_act.lf_id,(model_action.original_position).as_ref().unwrap() );
    match symbex_fire_action(gen_ctx,
                             exe_ctx,
                             current_diversity_ec_id,
                             client,
                             target_action_fqn,
                             variable_diversity_values).await {
        SymbexResult::UnSAT => {
            /*let fail_kind = HibouExecutionFailure::SolverUnSat(exe_ctx.clone(),
                                                               model_action.clone(),
                                                               None,
                                                               TD_Bool::FALSE,
                                                               None);*/
            return ModelSymbexResult::UnSat; //(fail_kind);
        },
        SymbexResult::Success( symbex_result ) => {
            // ***
            let mut lf_interpretation : BTreeMap<usize,TD_Generic>;
            match exe_ctx.get_lf_interpretation(model_action.lf_act.lf_id) {
                None => {
                    lf_interpretation = BTreeMap::new();
                },
                Some( got_lfint ) => {
                    lf_interpretation = got_lfint.clone();
                }
            }
            // ***
            {
                let mut variables_to_query_values: HashSet<usize> = appearing_variables.clone(); //.union(exe_ctx.get_active_clocks()).cloned().collect();
                for clock_id in exe_ctx.get_active_clocks() {
                    if lf_interpretation.contains_key(clock_id) {
                        variables_to_query_values.insert(*clock_id);
                    }
                }
                // ***
                for vr_id in &variables_to_query_values {
                    let vr_type = exe_ctx.get_vr_type(gen_ctx,*vr_id).unwrap();
                    let var_fqn = variable_diversity_fqn(gen_ctx,exe_ctx,model_action.lf_act.lf_id,*vr_id);
                    println!("HIBOU requested value of variable '{:?}' to DIVERSITY...",var_fqn);
                    let td_gen = symbex_request_variable(gen_ctx,exe_ctx,symbex_result.new_diversity_ec_id,client,var_fqn,&vr_type).await;
                    println!("...DIVERSITY provided value '{:?}'...",td_gen);
                    lf_interpretation.insert(*vr_id,td_gen);
                    println!("...updated in HIBOU intrepretation");
                }
            }
            // ***
            exe_ctx.set_lf_interpretation(model_action.lf_act.lf_id,lf_interpretation);

            /*
                HIBOU queries DIVERSITY
                for the values of the message parameters
            */
            let mut effective_parameters : Vec<TD_Generic> = Vec::new();
            let mut pr_id : usize = 0;
            for param in &model_action.params {
                let prm_type = gen_ctx.get_pr_type(model_action.ms_id, pr_id).unwrap();
                let prm_fqn = message_parameter_diversity_fqn(gen_ctx,exe_ctx,model_action.lf_act.lf_id,model_action.ms_id,pr_id);
                let td_gen = symbex_request_variable(gen_ctx,exe_ctx,symbex_result.new_diversity_ec_id,client,prm_fqn,&prm_type).await;
                effective_parameters.push( td_gen );
                pr_id = pr_id +1;
            }
            // ***
            match temporality {
                HibouProcessTemporality::UnTimed => {
                    return ModelSymbexResult::Sat( symbex_result.new_diversity_ec_id,
                                                   symbex_result.firing_condition,
                                                   effective_parameters,
                                                   None );
                },
                HibouProcessTemporality::Timed => {
                    println!("HIBOU requested delay symbol to DIVERSITY...");
                    let td_gen = symbex_request_variable(gen_ctx,exe_ctx,symbex_result.new_diversity_ec_id,client,"$delay".to_string(),&TD_DataType::Float).await;
                    println!("...DIVERSITY provided value '{:?}'...",td_gen);
                    match td_gen {
                        TD_Generic::Float(td_float) => {
                            return ModelSymbexResult::Sat( symbex_result.new_diversity_ec_id,
                                                           symbex_result.firing_condition,
                                                           effective_parameters,
                                                           Some(td_float) );
                        },
                        _ => {
                            panic!();
                        }
                    }
                }
            }

        }
    }
}


fn gather_appearing_variables(model_action : &ObservableAction) -> HashSet<usize> {
    let mut appearing_variables : HashSet<usize> = HashSet::new();
    // ***
    for amble_item in &model_action.lf_act.preamble {
        match amble_item {
            ActionAmbleItem::Guard(td_bool) => {
                appearing_variables.extend(td_bool.get_occuring_variables() );
            },
            ActionAmbleItem::Assignment( var_id, value_or_new_fresh) => {
                appearing_variables.insert( *var_id );
                match value_or_new_fresh {
                    ValueOrNewFresh::Value( td_gen ) => {
                        appearing_variables.extend(td_gen.get_occuring_variables() );
                    },
                    _ => {}
                }
            },
            ActionAmbleItem::Reset( var_id ) => {
                appearing_variables.insert( *var_id );
            },
            _ => {
                panic!();
            }
        }
    }
    // ***
    for value_or_new_fresh in &model_action.params {
        match value_or_new_fresh {
            ValueOrNewFresh::Value( ref td_gen ) => {
                appearing_variables.extend(td_gen.get_occuring_variables() );
            },
            _ => {}
        }
    }
    // ***
    for amble_item in &model_action.lf_act.postamble {
        match amble_item {
            ActionAmbleItem::Guard(td_bool) => {
                appearing_variables.extend(td_bool.get_occuring_variables() );
            },
            ActionAmbleItem::Assignment( var_id, value_or_new_fresh) => {
                appearing_variables.insert( *var_id );
                match value_or_new_fresh {
                    ValueOrNewFresh::Value( td_gen ) => {
                        appearing_variables.extend(td_gen.get_occuring_variables() );
                    },
                    _ => {}
                }
            },
            ActionAmbleItem::Reset( var_id ) => {
                appearing_variables.insert( *var_id );
            },
            _ => {
                panic!();
            }
        }
    }
    return appearing_variables;
}


