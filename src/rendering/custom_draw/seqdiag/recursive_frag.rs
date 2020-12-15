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

use crate::core::syntax::interaction::Interaction;


pub fn get_recursive_alt_frags(interaction : &Interaction) -> Vec<&Interaction> {
    let mut frags : Vec<&Interaction> = Vec::new();
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            frags.extend( get_recursive_alt_frags(i1));
            frags.extend( get_recursive_alt_frags(i2));
        },
        _ => {
            frags.push(interaction);
        }
    }
    return frags;
}

pub fn get_recursive_par_frags(interaction : &Interaction) -> Vec<&Interaction> {
    let mut frags : Vec<&Interaction> = Vec::new();
    match interaction {
        &Interaction::Par(ref i1, ref i2) => {
            frags.extend( get_recursive_par_frags(i1));
            frags.extend( get_recursive_par_frags(i2));
        },
        _ => {
            frags.push(interaction);
        }
    }
    return frags;
}

pub fn get_recursive_strict_frags(interaction : &Interaction) -> Vec<&Interaction> {
    let mut frags : Vec<&Interaction> = Vec::new();
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            frags.extend( get_recursive_strict_frags(i1));
            frags.extend( get_recursive_strict_frags(i2));
        },
        _ => {
            frags.push(interaction);
        }
    }
    return frags;
}

