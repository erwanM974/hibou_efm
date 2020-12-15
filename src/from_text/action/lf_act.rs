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

use pest::iterators::{Pair,Pairs};



use crate::core::syntax::action::*;
use crate::core::context::general::GeneralContext;

use crate::from_text::error::HibouParsingError;

use crate::from_text::parser::*;
use crate::from_text::action::amble::parse_amble;
use crate::from_text::data::generic::parse_data;


pub fn parse_lifeline_action(gen_ctx : &GeneralContext,
                            lifeline_action : Pair<Rule>,
                            opt_ms_id : &Option<usize>) -> Result<LifelineAction,HibouParsingError> {

    let preamble : Vec<ActionAmbleItem>;
    let postamble : Vec<ActionAmbleItem>;
    let lf_id : usize;
    // ***
    let mut lifeline_action_pairs = lifeline_action.into_inner();
    // ***
    let first_arg = lifeline_action_pairs.next().unwrap();
    match first_arg.as_rule() {
        Rule::SD_ACTION_AMBLE => {
            match parse_amble(gen_ctx, first_arg, &None) {
                Err(e) => {
                    return Err(e);
                },
                Ok( amble ) => {
                    preamble = amble;
                    let lf_name : String = lifeline_action_pairs.next().unwrap().as_str().chars().filter(|c| !c.is_whitespace()).collect();
                    match gen_ctx.get_lf_id( &lf_name) {
                        None => {
                            return Err(HibouParsingError::MissingLifelineDeclarationError(lf_name));
                        },
                        Some( got_lf_id ) => {
                            lf_id = got_lf_id;
                        }
                    }
                }
            }
        },
        Rule::LIFELINE_LABEL => {
            preamble = Vec::new();
            let lf_name : String = first_arg.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_lf_id( &lf_name) {
                None => {
                    return Err(HibouParsingError::MissingLifelineDeclarationError(lf_name));
                },
                Some( got_lf_id ) => {
                    lf_id = got_lf_id;
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", first_arg.as_rule() );
        }
    }
    // ***
    match lifeline_action_pairs.next() {
        None => {
            postamble = Vec::new();
        },
        Some( postamble_pair ) => {
            match parse_amble(gen_ctx, postamble_pair, opt_ms_id) {
                Err(e) => {
                    return Err(e);
                },
                Ok( amble ) => {
                    postamble = amble;
                }
            }
        }
    }
    // ***
    let lf_act = LifelineAction{preamble, lf_id, postamble};
    return Ok( lf_act );
}


