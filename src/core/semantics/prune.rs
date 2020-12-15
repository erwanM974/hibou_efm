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

use crate::core::syntax::position::*;
use crate::core::syntax::interaction::*;
use crate::core::semantics::frontier::*;


pub fn prune(my_int : &Interaction, lf_id : usize) -> Interaction {
    match my_int {
        Interaction::Empty => {
            return Interaction::Empty;
        },
        Interaction::Action(ref act) => {
            return Interaction::Action(act.clone());
        },
        Interaction::Seq(ref i1, ref i2) => {
            return Interaction::Seq( Box::new( prune(i1,lf_id)), Box::new( prune(i2,lf_id)) );
        },
        Interaction::Strict(ref i1, ref i2) => {
            return Interaction::Strict( Box::new( prune(i1,lf_id)), Box::new( prune(i2,lf_id)) );
        },
        Interaction::Par(ref i1, ref i2) => {
            return Interaction::Par( Box::new( prune(i1,lf_id)), Box::new( prune(i2,lf_id)) );
        },
        Interaction::Alt(ref i1, ref i2) => {
            if i1.avoids(lf_id) {
                if i2.avoids(lf_id) {
                    return Interaction::Alt( Box::new( prune(i1,lf_id)), Box::new( prune(i2,lf_id)) );
                } else {
                    return prune(i1,lf_id);
                }
            } else {
                return prune(i2,lf_id);
            }
        },
        Interaction::Loop(ref lkind, ref i1) => {
            if i1.avoids(lf_id) {
                return Interaction::Loop(lkind.clone(), Box::new(prune(i1,lf_id)));
            } else {
                return Interaction::Empty;
            }
        },
        Interaction::Scope(sko, i1) => {
            return Interaction::Scope(sko.clone(), Box::new(prune(i1,lf_id)));
        }
    }
}

