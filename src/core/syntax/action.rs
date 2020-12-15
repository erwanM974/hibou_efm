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

use std::fmt::Debug;
use std::collections::HashSet;

use crate::core::syntax::data::builtin::bool::TD_Bool;
use crate::core::syntax::data::generic::TD_Generic;

use crate::core::trace::{TraceAction,TraceActionKind};

#[derive(Clone, PartialEq, Debug)]
pub enum ValueOrNewFresh {
    Value(TD_Generic),
    NewFresh
}

#[derive(Clone, PartialEq, Debug)]
pub enum ActionAmbleItem {
    Guard(TD_Bool),
    Assignment(usize, ValueOrNewFresh),
    Reset(usize)
}

#[derive(Clone, PartialEq, Debug)]
pub struct LifelineAction { // <T : Clone + PartialEq + Debug>
    pub preamble : Vec<ActionAmbleItem>,
    pub lf_id : usize, // T,
    pub postamble : Vec<ActionAmbleItem>
}

#[derive(Clone, PartialEq, Debug)]
pub enum ObservableActionKind {
    Reception,
    Emission(Vec<LifelineAction>)
}

#[derive(Clone, PartialEq, Debug)]
pub struct ObservableAction {
    pub lf_act : LifelineAction,
    pub act_kind : ObservableActionKind,
    pub ms_id : usize,
    pub params : Vec<ValueOrNewFresh>,
    pub original_position : Option<Vec<u32>>
}


impl ObservableAction {

    pub fn get_action_kind(&self) -> TraceActionKind {
        match self.act_kind {
            ObservableActionKind::Reception => {
                return TraceActionKind::Reception;
            },
            ObservableActionKind::Emission(_) => {
                return TraceActionKind::Emission;
            }
        }
    }

    pub fn occupation_before(&self) -> usize {
        return self.lf_act.lf_id;
    }

    pub fn occupation_after(&self) -> HashSet<usize> {
        match self.act_kind {
            ObservableActionKind::Emission(ref targets) => {
                let mut occ : HashSet<usize> = HashSet::new();
                for targ in targets {
                    occ.insert(  targ.lf_id );
                }
                occ.insert( self.occupation_before() );
                return occ;
            },
            _ => {
                let mut occ : HashSet<usize> = HashSet::new();
                occ.insert( self.occupation_before() );
                return occ;
            }
        }
    }
    
}

