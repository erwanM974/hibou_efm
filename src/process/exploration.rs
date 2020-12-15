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
use std::fs;
use std::fs::File;
use std::io::{Read,BufReader,BufRead,BufWriter,Write};
use std::iter::FromIterator;

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;

use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;

use crate::core::syntax::data::builtin::integer::TD_Integer;
use crate::core::syntax::data::builtin::bool::TD_Bool;
use crate::core::syntax::data::generic::TD_Generic;

use crate::core::trace::*;
use crate::core::semantics::frontier::make_frontier;
use crate::core::semantics::shape_execute::shape_execute;

use crate::process::verdicts::*;
use crate::process::log::ProcessLogger;
use crate::process::hibou_process::*;
use crate::process::process_manager::*;
use crate::process::deploy_receptions::deploy_original_action_followup;

use crate::xlia::model::generate_xlia_model;

use crate::grpc_connect::calls::*;
use crate::grpc_connect::init_calls::*;
use crate::grpc_connect::post_process::*;
use crate::grpc_connect::to_grpc::{td_generic_to_grpc,td_bool_to_grpc};
use crate::grpc_connect::xlia_reference_name_tools::*;

use crate::diversity::symbex_client::SymbexClient;
use crate::diversity::*;

use crate::process::queue::ProcessQueue;

pub async fn explore(interaction : Interaction,
                     gen_ctx : GeneralContext,
                     exe_ctx : ExecutionContext,
                     temporality : HibouProcessTemporality,
                     pre_filters : Vec<HibouPreFilter>,
                     strategy : HibouSearchStrategy,
                     frontier_priorities : ProcessPriorities,
                     loggers : Vec<Box<dyn ProcessLogger>>) {
    // ***
    let mut first_context = exe_ctx;
    let xlia_model_string = generate_xlia_model(&gen_ctx,&first_context,&interaction, &temporality);
    let model_file_path = "xlia_model.xlia".to_string();
    let mut file = File::create(&model_file_path).unwrap();
    file.write( xlia_model_string.as_bytes() );
    println!("generated xlia model :\n{}",xlia_model_string);
    // ***
    let (mut client,mut initial_div_ec_id) = symbex_init_model(&gen_ctx,&mut first_context,xlia_model_string).await;
    // ***
    initial_div_ec_id = symbex_fire_lifeline_initializations(&mut client, &gen_ctx,&mut first_context,initial_div_ec_id).await;
    // ***
    let mut manager = HibouProcessManager::new(gen_ctx,
                                               strategy,
                                               temporality,
                                               pre_filters,
                                               HashMap::new(),
                                               ProcessQueue::new(),
                                               frontier_priorities,
                                               loggers);
    // ***
    manager.init_loggers(&first_context,&interaction,&None);
    // ***
    let mut next_state_id : u32 = 1;
    let mut node_counter : u32 = 0;
    enqueue_next_node_in_exploration(&mut manager,
                                     next_state_id,
                                     initial_div_ec_id,
                                     first_context,
                                     interaction,0,0);
    next_state_id = next_state_id + 1;
    node_counter = node_counter +1;
    // ***
    // ***
    while let Some(next_to_process) = manager.extract_from_queue() {
        let new_state_id = next_state_id;
        next_state_id = next_state_id + 1;
        // ***
        let mut parent_state = manager.get_memorized_state(next_to_process.state_id).unwrap().clone();
        // ***
        match manager.process_next(&mut client,
                                   &parent_state,
                                   &next_to_process,
                                   new_state_id,
                                   node_counter).await {
            None => {},
            Some( (new_interaction,new_exe_ctx,new_div_ec_id,new_multi_trace,new_depth,new_loop_depth) ) => {
                node_counter = node_counter + 1;
                enqueue_next_node_in_exploration(&mut manager,
                                                 new_state_id,
                                                 new_div_ec_id,
                                                 new_exe_ctx,
                                                 new_interaction,
                                                 new_depth,
                                                 new_loop_depth);
            }
        }
        // ***
        parent_state.remaining_ids_to_process.remove(&next_to_process.id_as_child);
        if parent_state.remaining_ids_to_process.len() == 0 {
            manager.forget_state(next_to_process.state_id);
        } else {
            manager.remember_state(next_to_process.state_id,parent_state);
        }
        // ***
    }
    // ***
    symbex_post_process(&mut client).await;
    // ***
    manager.term_loggers(None);
    // ***
}



fn enqueue_next_node_in_exploration(manager      : &mut HibouProcessManager,
                                     state_id    : u32,
                                     diversity_ec_id : u32,
                                     exe_ctx : ExecutionContext,
                                     interaction : Interaction,
                                     depth       : u32,
                                     loop_depth  : u32) {
    // ***
    let mut next_child_id : u32 = 0;
    // ***
    let mut to_enqueue : Vec<(u32,NextToProcessKind)> = Vec::new();
    for front_pos in make_frontier(&interaction) {
        next_child_id = next_child_id +1;
        let child_kind = NextToProcessKind::Execute(front_pos);
        to_enqueue.push( (next_child_id,child_kind) );
    }
    // ***
    if next_child_id > 0 {
        let rem_child_ids : HashSet<u32> = HashSet::from_iter((1..(next_child_id+1)).collect::<Vec<u32>>().iter().cloned() );
        let memo_state = MemorizedState::new(interaction,
                                             exe_ctx,
                                             diversity_ec_id,
                                             None,
                                             rem_child_ids,
                                             loop_depth,depth);
        manager.remember_state( state_id, memo_state );
        manager.enqueue_executions( state_id, to_enqueue );
    }
}



