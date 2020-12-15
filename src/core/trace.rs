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

use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::builtin::float::TD_Float;

use crate::core::syntax::action::ObservableAction;

#[derive(Clone, PartialEq, Debug)]
pub enum TraceActionKind {
    Reception,
    Emission
}

#[derive(Clone, PartialEq, Debug)]
pub struct TraceAction {
    pub delay : Option<TD_Float>,
    pub lf_id : usize,
    pub act_kind : TraceActionKind,
    pub ms_id : usize,
    pub arguments : Vec<TD_Generic>
}

impl TraceAction {

    pub fn is_signature_match(&self, model_action: &ObservableAction) -> bool {
        if model_action.lf_act.lf_id != self.lf_id {
            return false;
        }
        if model_action.ms_id != self.ms_id {
            return false;
        }
        if self.act_kind != model_action.get_action_kind() {
            return false;
        }
        if self.arguments.len() != model_action.params.len() {
            return false;
        }
        return true;
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct MultiTraceCanal {
    pub lifelines : HashSet<usize>,
    pub trace : Vec<TraceAction>
}

#[derive(Clone, PartialEq, Debug)]
pub struct AnalysableMultiTrace {
    pub canals : Vec<MultiTraceCanal>
}

impl AnalysableMultiTrace {
    pub fn new(canals:Vec<MultiTraceCanal>) -> AnalysableMultiTrace {
        return AnalysableMultiTrace{canals};
    }

    pub fn length(&self) -> usize {
        let mut length = 0;
        for canal in &self.canals {
            length = length + (canal.trace.len());
        }
        return length;
    }

    pub fn is_any_component_empty(&self) -> bool {
        for canal in &self.canals {
            if canal.trace.len() == 0 {
                return true;
            }
        }
        return false;
    }

}



