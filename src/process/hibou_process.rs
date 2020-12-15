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

use std::cmp::Reverse;
use std::collections::{HashSet,HashMap};

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::*;
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::make_frontier;
use crate::core::semantics::shape_execute::shape_execute;

use crate::core::syntax::data::builtin::bool::TD_Bool;


#[derive(Clone, PartialEq, Debug)]
pub struct MemorizedState {
    pub interaction : Interaction,
    pub exe_ctx : ExecutionContext,
    pub diversity_ec_id : u32,
    pub multi_trace : Option<AnalysableMultiTrace>,
    pub remaining_ids_to_process : HashSet<u32>,
    pub loop_depth : u32, // number of loop instanciations since intial interaction
    pub depth : u32       // number of execution steps since initial interaction
}

impl MemorizedState {
    pub fn new(interaction : Interaction,
               exe_ctx : ExecutionContext,
               diversity_ec_id : u32,
               multi_trace : Option<AnalysableMultiTrace>,
               remaining_ids_to_process : HashSet<u32>,
               loop_depth : u32,
               depth : u32) -> MemorizedState {
        return MemorizedState{interaction,exe_ctx,diversity_ec_id,multi_trace,remaining_ids_to_process,loop_depth,depth};
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum NextToProcessKind {
    Execute(Position)
}

#[derive(Clone, PartialEq, Debug)]
pub struct NextToProcess {
    pub state_id : u32,
    pub id_as_child : u32,
    pub kind : NextToProcessKind
}

impl NextToProcess {
    pub fn new(state_id : u32,
               id_as_child : u32,
               kind : NextToProcessKind) -> NextToProcess {
        return NextToProcess{state_id,id_as_child,kind};
    }
}



pub enum HibouPreFilter {
    MaxLoopInstanciation(u32),
    MaxProcessDepth(u32),
    MaxNodeNumber(u32)
}

impl std::string::ToString for HibouPreFilter {
    fn to_string(&self) -> String {
        match self {
            HibouPreFilter::MaxLoopInstanciation(num) => {
                return format!("MaxLoop={}",num);
            },
            HibouPreFilter::MaxProcessDepth(num) => {
                return format!("MaxDepth={}",num);
            },
            HibouPreFilter::MaxNodeNumber(num) => {
                return format!("MaxNum={}",num);
            }
        }
    }
}

pub enum FilterEliminationKind {
    MaxLoopInstanciation,
    MaxProcessDepth,
    MaxNodeNumber
}

impl std::string::ToString for FilterEliminationKind {
    fn to_string(&self) -> String {
        match self {
            FilterEliminationKind::MaxLoopInstanciation => {
                return "MaxLoop".to_string();
            },
            FilterEliminationKind::MaxProcessDepth => {
                return "MaxDepth".to_string();
            },
            FilterEliminationKind::MaxNodeNumber => {
                return "MaxNum".to_string();
            }
        }
    }
}


pub enum HibouSearchStrategy {
    BFS,
    DFS
}

impl std::string::ToString for HibouSearchStrategy {
    fn to_string(&self) -> String {
        match self {
            HibouSearchStrategy::BFS => {
                return "Breadth First Search".to_string();
            },
            HibouSearchStrategy::DFS => {
                return "Depth First Search".to_string();
            }
        }
    }
}


pub enum HibouProcessTemporality {
    Timed,
    UnTimed
}

impl std::string::ToString for HibouProcessTemporality {
    fn to_string(&self) -> String {
        match self {
            HibouProcessTemporality::Timed => {
                return "Timed".to_string();
            },
            HibouProcessTemporality::UnTimed => {
                return "UnTimed".to_string();
            }
        }
    }
}

























