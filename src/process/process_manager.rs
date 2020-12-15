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


use std::collections::HashMap;
use std::cmp::Reverse;

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;

use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::{AnalysableMultiTrace,MultiTraceCanal,TraceAction};
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::make_frontier;
use crate::core::semantics::shape_execute::shape_execute;
use crate::process::verdicts::*;
use crate::process::hibou_process::*;
use crate::core::trace::*;
use crate::process::queue::ProcessQueue;

use crate::process::symbex::model_symbex::*;
use crate::process::symbex::trace_symbex::*;
use crate::process::deploy_receptions::deploy_original_action_followup;

use crate::diversity::symbex_client::SymbexClient;
use crate::diversity::*;

pub struct ProcessPriorities {
    pub emission : i32,
    pub reception : i32,
    pub in_loop : i32
}

impl ProcessPriorities {
    pub fn new(emission : i32,
               reception : i32,
               in_loop : i32) -> ProcessPriorities {
        return ProcessPriorities{emission,reception,in_loop};
    }
}

impl std::string::ToString for ProcessPriorities {
    fn to_string(&self) -> String {
        let mut my_str = format!("emission={:},",self.emission);
        my_str.push_str( &format!("reception={:},",self.reception));
        my_str.push_str( &format!("in_loop={:}",self.in_loop));
        return my_str;
    }
}

pub struct HibouProcessManager {
    gen_ctx : GeneralContext,
    strategy : HibouSearchStrategy,
    temporality : HibouProcessTemporality,
    pre_filters : Vec<HibouPreFilter>,
    // ***
    memorized_states : HashMap<u32,MemorizedState>,
    process_queue : ProcessQueue,
    // ***
    frontier_priorities : ProcessPriorities,
    // ***
    loggers : Vec<Box<dyn ProcessLogger>>
}

impl HibouProcessManager {
    pub fn new(gen_ctx : GeneralContext,
               strategy : HibouSearchStrategy,
               temporality : HibouProcessTemporality,
               pre_filters : Vec<HibouPreFilter>,
               memorized_states : HashMap<u32,MemorizedState>,
               process_queue : ProcessQueue,
               frontier_priorities : ProcessPriorities,
               loggers : Vec<Box<dyn ProcessLogger>>
    ) -> HibouProcessManager {
        return HibouProcessManager{gen_ctx,strategy,temporality,pre_filters,memorized_states,process_queue,frontier_priorities,loggers};
    }

    pub fn get_options_as_strings(&self,goal_and_verdict:Option<(&GlobalVerdict,&GlobalVerdict)>) -> Vec<String> {
        let mut options_str : Vec<String> = Vec::new();
        match goal_and_verdict {
            None => {
                options_str.push("process=exploration".to_string());
            },
            Some( (goal,verd) ) => {
                options_str.push("process=analysis".to_string());
                options_str.push( format!("goal={}", goal.to_string()) );
                options_str.push( format!("verdict={}", verd.to_string()) );
            }
        }
        options_str.push( format!("temporality={}", &self.temporality.to_string()) );
        options_str.push( format!("strategy={}", &self.strategy.to_string()) );
        options_str.push( format!("frontier_priorities=[{}]", &self.frontier_priorities.to_string()) );
        {
            let mut rem_filter = self.pre_filters.len();
            let mut filters_str = "filters=[".to_string();
            for filter in &self.pre_filters {
                filters_str.push_str( &filter.to_string() );
                rem_filter = rem_filter - 1;
                if rem_filter > 0 {
                    filters_str.push_str( "," );
                }
            }
            filters_str.push_str( "]" );
            options_str.push( filters_str );
        }
        return options_str;
    }

    pub fn init_loggers(&mut self,
                        exe_ctx : &ExecutionContext,
                        interaction : &Interaction,
                        remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_init(interaction, &self.gen_ctx, exe_ctx, remaining_multi_trace);
        }
    }

    pub fn term_loggers(&mut self,
                        goal_and_verdict:Option<(&GlobalVerdict,&GlobalVerdict)>) {
        let options_as_strs = (&self).get_options_as_strings(goal_and_verdict);
        for logger in self.loggers.iter_mut() {
            (*logger).log_term(&options_as_strs);
        }
    }

    pub fn verdict_loggers(&mut self,
                           verdict : &CoverageVerdict,
                           parent_state_id : u32) {
        for logger in self.loggers.iter_mut() {
            logger.log_verdict(parent_state_id,
                               verdict);
        }
    }

    pub fn filtered_loggers(&mut self,
                            action_position : &Position,
                            executed_action : &ObservableAction,
                            parent_state_id : u32,
                            new_state_id : u32,
                            elim_kind : &FilterEliminationKind) {
        let parent_state = self.memorized_states.get(&parent_state_id).unwrap();
        for logger in self.loggers.iter_mut() {
            logger.log_filtered(&self.gen_ctx,
                                &parent_state.exe_ctx,
                                parent_state_id,
                                new_state_id,
                                action_position,
                                executed_action,
                                elim_kind);
        }
    }

    pub fn unsat_loggers(&mut self,
                            action_position : &Position,
                            model_action : &ObservableAction,
                            trace_action : Option<&TraceAction>,
                            parent_state_id : u32,
                            new_state_id : u32) {
        let parent_state = self.memorized_states.get(&parent_state_id).unwrap();
        for logger in self.loggers.iter_mut() {
            logger.log_unsat(&self.gen_ctx,
                                &parent_state.exe_ctx,
                                parent_state_id,
                                new_state_id,
                                action_position,
                             trace_action,
                             model_action);
        }
    }

    pub fn execution_loggers(&mut self,
                             action_position : &Position,
                             model_action : &ObservableAction,
                             trace_action : Option<&TraceAction>,
                            new_interaction : &Interaction,
                            new_exe_ctx : &ExecutionContext,
                            parent_state_id : u32,
                            new_state_id :u32,
                            remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        for logger in self.loggers.iter_mut() {
            logger.log_execution(&self.gen_ctx,
                                 parent_state_id,
                                 new_state_id,
                                 action_position,
                                 trace_action,
                                 model_action,
                                 new_interaction,
                                 new_exe_ctx,
                                 remaining_multi_trace);
        }
    }

    pub fn get_memorized_state(&self, id:u32) -> Option<&MemorizedState> {
        return self.memorized_states.get(&id);
    }

    pub fn forget_state(&mut self, id:u32) {
        self.memorized_states.remove(&id);
    }

    pub fn remember_state(&mut self, id:u32, state:MemorizedState) {
        self.memorized_states.insert( id, state );
    }

    pub fn extract_from_queue(&mut self) -> Option<NextToProcess> {
        return self.process_queue.get_next();
    }

    pub fn enqueue_executions(&mut self, state_id : u32, to_enqueue : Vec<(u32,NextToProcessKind)>) {
        let mut to_enqueue_reorganize : HashMap<i32,Vec<(u32,NextToProcessKind)>> = HashMap::new();
        for (child_id,child_kind) in to_enqueue {
            match &child_kind {
                &NextToProcessKind::Execute( ref front_pos ) => {
                    let mut priority : i32 = 0;
                    // ***
                    let parent_state = self.get_memorized_state(state_id).unwrap();
                    let front_act = (parent_state.interaction).get_sub_interaction(&front_pos).as_leaf();
                    match front_act.act_kind {
                        ObservableActionKind::Reception => {
                            priority = priority + self.frontier_priorities.reception;
                        },
                        ObservableActionKind::Emission(_) => {
                            priority = priority + self.frontier_priorities.emission;
                        }
                    }
                    let loop_depth = (parent_state.interaction).get_loop_depth_at_pos(&front_pos);
                    if loop_depth > 0 {
                        priority = priority + self.frontier_priorities.in_loop;
                    }
                    // ***
                    match to_enqueue_reorganize.get_mut(&priority) {
                        None => {
                            to_enqueue_reorganize.insert(priority,vec![ (child_id,child_kind) ]);
                        },
                        Some( queue ) => {
                            queue.push((child_id,child_kind) );
                        }
                    }
                    // ***
                }
            }
        }
        // ***
        let mut to_enqueue_reorganized : Vec<(u32,NextToProcessKind)> = Vec::new();
        {
            let mut keys : Vec<i32> = to_enqueue_reorganize.keys().cloned().collect();
            keys.sort_by_key(|k| Reverse(*k));
            for k in keys {
                match to_enqueue_reorganize.get_mut(&k) {
                    None => {},
                    Some( queue ) => {
                        to_enqueue_reorganized.append( queue );
                    }
                }
            }
        }
        // ***
        match &self.strategy {
            &HibouSearchStrategy::DFS => {
                to_enqueue_reorganized.reverse();
                for (child_id,child_kind) in to_enqueue_reorganized {
                    self.enqueue_child_node(state_id,child_id,child_kind);
                }
            },
            &HibouSearchStrategy::BFS => {
                for (child_id,child_kind) in to_enqueue_reorganized {
                    self.enqueue_child_node(state_id,child_id,child_kind);
                }
            }
        }
    }

    fn enqueue_child_node(&mut self,state_id: u32,child_id:u32,child_kind:NextToProcessKind) {
        let child = NextToProcess::new(state_id,child_id,child_kind);
        match &(self.strategy) {
            &HibouSearchStrategy::DFS => {
                self.process_queue.insert_item_left(child);
            },
            &HibouSearchStrategy::BFS => {
                self.process_queue.insert_item_right(child);
            }
        }
    }

    pub async fn process_next(&mut self,
                              client : &mut SymbexClient<tonic::transport::Channel>,
                        parent_state : &MemorizedState,
                        to_process   : &NextToProcess,
                        new_state_id : u32,
                        node_counter : u32) -> Option<(Interaction,ExecutionContext,u32,Option<AnalysableMultiTrace>,u32,u32)> {
        match &(to_process.kind) {
            &NextToProcessKind::Execute( ref position ) => {
                let new_depth = parent_state.depth + 1;
                let new_loop_depth = parent_state.loop_depth + (parent_state.interaction).get_loop_depth_at_pos(position);
                // ***
                match self.apply_pre_filters(new_depth,new_loop_depth,node_counter) {
                    None => {
                        let mut new_exe_ctx = parent_state.exe_ctx.clone();
                        match shape_execute(&self.gen_ctx,&mut new_exe_ctx,&parent_state.interaction,position) {
                            Err(e) => {
                                panic!("{:?}",e);
                            },
                            Ok( (shaped_interaction,shaped_position,shaped_action,needs_scoping) ) => {
                                match model_symbolic_execution(client,
                                                               &self.gen_ctx,
                                                               &mut new_exe_ctx,
                                                               &shaped_action,
                                                               parent_state.diversity_ec_id,
                                                               needs_scoping,
                                                               &self.temporality).await {
                                    ModelSymbexResult::UnSat => {
                                        self.unsat_loggers(&position,
                                                           &shaped_action,
                                                           None,
                                                           to_process.state_id,
                                                           new_state_id);
                                        return None;
                                    },
                                    ModelSymbexResult::Sat( new_diversity_ec_id,
                                                            model_firing_conditions,
                                                            effective_parameters,
                                                            opt_delay ) => {
                                        // ***
                                        match (parent_state.multi_trace).as_ref(){
                                            None => {
                                                let new_interaction = deploy_original_action_followup(&new_exe_ctx,
                                                                                                  &shaped_interaction,
                                                                                                  &shaped_position,
                                                                                                  &shaped_action,
                                                                                                  &effective_parameters);
                                                // ***
                                                let trace_act_kind : TraceActionKind;
                                                match &shaped_action.act_kind {
                                                    ObservableActionKind::Reception => {
                                                        trace_act_kind = TraceActionKind::Reception;
                                                    },
                                                    ObservableActionKind::Emission(_) => {
                                                        trace_act_kind = TraceActionKind::Emission;
                                                    }
                                                }
                                                let trace_action = TraceAction{ delay:opt_delay,
                                                    lf_id:shaped_action.lf_act.lf_id,
                                                    ms_id:shaped_action.ms_id,
                                                    act_kind:trace_act_kind,
                                                    arguments:effective_parameters};
                                                // ***
                                                self.execution_loggers(&position,
                                                                       &shaped_action,
                                                                       Some(&trace_action),
                                                                       &new_interaction,
                                                                       &new_exe_ctx,
                                                                       to_process.state_id,
                                                                       new_state_id,
                                                                       &None);
                                                // ***
                                                return Some( (new_interaction,new_exe_ctx,new_diversity_ec_id,None,new_depth,new_loop_depth) );
                                            },
                                            Some( ref multi_trace ) => {
                                                let new_multi_trace : Option<AnalysableMultiTrace>;
                                                let mut head_trace_action_opt : Option<TraceAction> = None;
                                                {
                                                    let mut new_canals : Vec<MultiTraceCanal> = Vec::new();
                                                    for canal in &multi_trace.canals {
                                                        if canal.lifelines.contains(&shaped_action.occupation_before()) {
                                                            let mut new_trace = canal.trace.clone();
                                                            head_trace_action_opt = Some(new_trace.remove(0));
                                                            new_canals.push( MultiTraceCanal{lifelines:canal.lifelines.clone(),trace:new_trace} )
                                                        } else {
                                                            new_canals.push(canal.clone());
                                                        }
                                                    }
                                                    new_multi_trace = Some( AnalysableMultiTrace::new(new_canals) );
                                                }
                                                let head_trace_action = head_trace_action_opt.unwrap();
                                                // ***
                                                match trace_symbolic_execution(client,
                                                                               &self.gen_ctx,
                                                                               &mut new_exe_ctx,
                                                                               shaped_action.lf_act.lf_id,
                                                                               shaped_action.ms_id,
                                                                               &head_trace_action.arguments,
                                                                               &head_trace_action.delay,
                                                                               &self.temporality,
                                                                               new_diversity_ec_id).await {
                                                    TraceSymbexResult::UnSat(trace_firing_conditions) => {
                                                        self.unsat_loggers(&position,
                                                                           &shaped_action,
                                                                           Some(&head_trace_action),
                                                                           to_process.state_id,
                                                                           new_state_id);
                                                        return None;
                                                    },
                                                    TraceSymbexResult::Sat(post_trace_analysis_diversity_ec_id,trace_firing_condition) => {
                                                        let post_trace_analysis_interaction = deploy_original_action_followup(&new_exe_ctx,
                                                                                                                              &shaped_interaction,
                                                                                                                              &shaped_position,
                                                                                                                              &shaped_action,
                                                                                                                              &head_trace_action.arguments);
                                                        // ***
                                                        self.execution_loggers(&position,
                                                                               &shaped_action,
                                                                               Some(&head_trace_action),
                                                                               &post_trace_analysis_interaction,
                                                                               &new_exe_ctx,
                                                                               to_process.state_id,
                                                                               new_state_id,
                                                                               &new_multi_trace);
                                                        // ***
                                                        return Some( (post_trace_analysis_interaction,
                                                                      new_exe_ctx,
                                                                      post_trace_analysis_diversity_ec_id,
                                                                      new_multi_trace,
                                                                      new_depth,
                                                                      new_loop_depth) );
                                                    }
                                                }
                                                // ***
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some( elim_kind ) => {
                        let executed_action = (parent_state.interaction).get_sub_interaction(position).as_leaf();
                        self.filtered_loggers(&position,
                                              executed_action,
                                              to_process.state_id,
                                              new_state_id,
                                              &elim_kind);
                        return None;
                    }
                }
            },
            _ => {
                return None;
            }
        }
    }

    fn apply_pre_filters(&self, depth : u32, loop_depth : u32, node_counter : u32) -> Option<FilterEliminationKind> {
        for pre_filter in &self.pre_filters {
            match pre_filter {
                HibouPreFilter::MaxProcessDepth( max_depth ) => {
                    if depth > *max_depth {
                        return Some( FilterEliminationKind::MaxProcessDepth );
                    }
                },
                HibouPreFilter::MaxLoopInstanciation( loop_num ) => {
                    if loop_depth > *loop_num {
                        return Some( FilterEliminationKind::MaxLoopInstanciation );
                    }
                },
                HibouPreFilter::MaxNodeNumber( max_node_number ) => {
                    if node_counter >= *max_node_number {
                        return Some( FilterEliminationKind::MaxNodeNumber );
                    }
                }
            }
        }
        return None;
    }

    pub fn get_coverage_verdict(&self,interaction:&Interaction,multi_trace:&AnalysableMultiTrace) -> CoverageVerdict {
        if multi_trace.length() == 0 {
            if interaction.express_empty() {
                return CoverageVerdict::Cov;
            } else {
                return CoverageVerdict::TooShort;
            }
        } else {
            if multi_trace.is_any_component_empty() {
                return CoverageVerdict::LackObs;
            } else {
                return CoverageVerdict::Out;
            }
        }
    }

}