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

use crate::core::syntax::position::*;

pub fn position_to_text(position : &Position) -> String {
    match position {
        Position::Left(ref in_self) => {
            let mut my_string = SYNTAX_POSITION_LEFT.to_string();
            let sub_pos = position_to_text( &(*in_self) );
            if sub_pos != SYNTAX_POSITION_EPSILON.to_string() {
                my_string.push_str( &sub_pos );
            }
            return my_string;
        },
        Position::Right(ref in_self) => {
            let mut my_string = SYNTAX_POSITION_RIGHT.to_string();
            let sub_pos = position_to_text( &(*in_self) );
            if sub_pos != SYNTAX_POSITION_EPSILON.to_string() {
                my_string.push_str( &sub_pos );
            }
            return my_string;
        },
        Position::Epsilon => {
            return SYNTAX_POSITION_EPSILON.to_string();
        }
    }
}