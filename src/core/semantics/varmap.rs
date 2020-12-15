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

use std::collections::{HashSet,HashMap};

use crate::core::syntax::action::*;
use crate::core::syntax::interaction::Interaction;

pub trait VarMapAble {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> Self;
}


// ********** ********** ********** ********** ********** ********** **********
// ********** ********** ********** ********** ********** ********** **********
// ********** ********** ********** ********** ********** ********** **********

// For Interactions

impl VarMapAble for Interaction {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> Interaction {
        match self {
            &Interaction::Empty => {
                return Interaction::Empty;
            },
            &Interaction::Action(ref act) => {
                return Interaction::Action( act.apply_variable_mapping(mapping) );
            },
            &Interaction::Strict(ref i1, ref i2) => {
                return Interaction::Strict(Box::new(i1.apply_variable_mapping(mapping)),
                                           Box::new(i2.apply_variable_mapping(mapping)));
            },
            &Interaction::Seq(ref i1, ref i2) => {
                return Interaction::Seq(Box::new(i1.apply_variable_mapping(mapping)),
                                           Box::new(i2.apply_variable_mapping(mapping)));
            },
            &Interaction::Alt(ref i1, ref i2) => {
                return Interaction::Alt(Box::new(i1.apply_variable_mapping(mapping)),
                                           Box::new(i2.apply_variable_mapping(mapping)));
            },
            &Interaction::Par(ref i1, ref i2) => {
                return Interaction::Par(Box::new(i1.apply_variable_mapping(mapping)),
                                           Box::new(i2.apply_variable_mapping(mapping)));
            },
            &Interaction::Loop(ref lkind, ref i1) => {
                return Interaction::Loop(lkind.clone(),
                                         Box::new(i1.apply_variable_mapping(mapping)));
            },
            &Interaction::Scope(ref scoped_vr_ids, ref i1) => {
                let mut new_vr_ids : Vec<usize> = Vec::new();
                for vr_id in scoped_vr_ids {
                    match mapping.get(vr_id) {
                        None => {
                            new_vr_ids.push( *vr_id );
                        },
                        Some( new_vr_id ) => {
                            new_vr_ids.push( *new_vr_id );
                        }
                    }
                }
                return Interaction::Scope(new_vr_ids,
                                          Box::new(i1.apply_variable_mapping(mapping)));
            }
        }
    }
}

// ********** ********** ********** ********** ********** ********** **********
// ********** ********** ********** ********** ********** ********** **********
// ********** ********** ********** ********** ********** ********** **********

// For Actions and contents

impl VarMapAble for ActionAmbleItem {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> ActionAmbleItem {
        match self {
            ActionAmbleItem::Guard( guard ) => {
                return ActionAmbleItem::Guard( guard.apply_variable_mapping(mapping) );
            },
            ActionAmbleItem::Reset(vr_id) => {
                match mapping.get(vr_id) {
                    None => {
                        return ActionAmbleItem::Reset(*vr_id);
                    },
                    Some( new_vr_id ) => {
                        return ActionAmbleItem::Reset(*new_vr_id);
                    }
                }
            },
            ActionAmbleItem::Assignment(vr_id, valueornewfresh) => {
                let new_vr_id : usize;
                match mapping.get(vr_id) {
                    None => {
                        new_vr_id = *vr_id;
                    },
                    Some( npid ) => {
                        new_vr_id = *npid;
                    }
                }
                match valueornewfresh {
                    ValueOrNewFresh::Value( td_generic ) => {
                        return ActionAmbleItem::Assignment(new_vr_id,ValueOrNewFresh::Value(td_generic.apply_variable_mapping(mapping)));
                    },
                    ValueOrNewFresh::NewFresh => {
                        return ActionAmbleItem::Assignment(new_vr_id,ValueOrNewFresh::NewFresh);
                    }
                }
            }
        }
    }
}

impl VarMapAble for LifelineAction {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> LifelineAction {
        let mut preamble : Vec<ActionAmbleItem> = Vec::new();
        for item in &self.preamble {
            preamble.push( item.apply_variable_mapping(mapping) );
        }
        // ***
        let mut postamble : Vec<ActionAmbleItem> = Vec::new();
        for item in &self.postamble {
            postamble.push( item.apply_variable_mapping(mapping) );
        }
        // ***
        return LifelineAction{preamble,lf_id:self.lf_id,postamble};
    }
}


impl VarMapAble for ObservableAction {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> ObservableAction {
        // ***
        let new_lf_act = self.lf_act.apply_variable_mapping(mapping);
        // ***
        let new_act_kind :ObservableActionKind;
        match &self.act_kind {
            ObservableActionKind::Reception => {
                new_act_kind = ObservableActionKind::Reception;
            },
            ObservableActionKind::Emission(targets) => {
                let mut new_targets : Vec<LifelineAction> = Vec::new();
                for lf_act in targets {
                    new_targets.push( lf_act.apply_variable_mapping(mapping) );
                }
                new_act_kind = ObservableActionKind::Emission(new_targets);
            }
        }
        // ***
        let mut new_params : Vec<ValueOrNewFresh> = Vec::new();
        for par_arg in &self.params {
            match par_arg {
                ValueOrNewFresh::NewFresh => {
                    new_params.push( ValueOrNewFresh::NewFresh );
                },
                ValueOrNewFresh::Value( td_gen ) => {
                    new_params.push( ValueOrNewFresh::Value( td_gen.apply_variable_mapping(mapping) ));
                }
            }
        }
        // ***
        return ObservableAction{
            lf_act:new_lf_act,
            act_kind:new_act_kind,
            ms_id:self.ms_id,
            params:new_params,
            original_position:self.original_position.clone()
        }
    }
}

// ********** ********** ********** ********** ********** ********** **********
// ********** ********** ********** ********** ********** ********** **********
// ********** ********** ********** ********** ********** ********** **********

