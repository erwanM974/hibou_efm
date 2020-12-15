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

use std::fmt;

use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::position::Position;

use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::builtin::bool::TD_Bool;


use crate::diversity::*;


#[derive(Debug)]
pub enum HibouCoreError {
    PruningError,
    PositionError(Interaction,Position),
    UnknownLifelineGroup(usize),
    UnknownLifeline(usize),
    UnknownMessage(usize),
    UnknownParameter(usize),
    UnknownSymbol(Option<usize>,Option<String>),
    UninterpretedVariable(usize),
    UninterpretedParameter(usize,usize),
    WronglyTypedExpression(TD_Generic,TD_DataType),
    WronglyTypedGrpcInputOperation(Operation),
    WronglyTypedGrpcInput(String, TD_DataType, String),
    UnknownOperatorInGrpcInputOperation(Operation),
    SolverUnknownSatisfiability
}

impl fmt::Display for HibouCoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HibouCoreError::PruningError => {
                return write!(f,  "{}", format!("error while pruning"));
            },
            HibouCoreError::PositionError(interaction, position) => {
                return write!(f,  "{}", format!("position {:?} is not a leaf of {:?}", position, interaction));
            },
            HibouCoreError::UnknownLifelineGroup( lgr_id ) => {
                return write!(f, "{}", format!("context error ; unknown lifeline group : {:}", lgr_id));
            },
            HibouCoreError::UnknownLifeline( lf_id ) => {
                return write!(f, "{}", format!("context error ; unknown lifeline : {:}", lf_id));
            },
            HibouCoreError::UnknownMessage( ms_id ) => {
                return write!(f, "{}", format!("context error ; unknown message : {:}", ms_id));
            },
            HibouCoreError::UnknownParameter( vr_id ) => {
                return write!(f, "{}", format!("context error ; unknown parameter : {:}", vr_id));
            },
            HibouCoreError::UnknownSymbol( sy_id_opt, sy_fqn_opt ) => {
                match sy_id_opt {
                    None => {
                        match sy_fqn_opt {
                            None => {
                                panic!();
                            },
                            Some( sy_fqn ) => {
                                return write!(f, "{}", format!("context error ; unknown symbol fqn : {:}", sy_fqn));
                            }
                        }
                    },
                    Some(sy_id) => {
                        return write!(f, "{}", format!("context error ; unknown symbol id : {:}", sy_id));
                    }
                }
            },
            HibouCoreError::UninterpretedVariable( vr_id ) => {
                return write!(f, "{}", format!("context error ; uninterpreted variable : {:}", vr_id));
            },
            HibouCoreError::UninterpretedParameter( ms_id, pr_id ) => {
                return write!(f, "{}", format!("context error ; uninterpreted parameter {:} on message {:}", pr_id, ms_id));
            },
            HibouCoreError::WronglyTypedExpression( expr, expected_type ) => {
                return write!(f, "{}", format!("context error ; parameter : wrongly typed expression : {:?} - expected : {:?}", expr, expected_type));
            },
            HibouCoreError::WronglyTypedGrpcInputOperation( operation ) => {
                return write!(f, "{}", format!("grpc input error on operation : {:?}", operation));
            },
            HibouCoreError::WronglyTypedGrpcInput( input_as_str, expected_type, interpreted_type ) => {
                return write!(f, "{}", format!("grpc input error in '{:?}' - expected '{:?}' - got '{:?}'", input_as_str, expected_type, interpreted_type ));
            },
            HibouCoreError::UnknownOperatorInGrpcInputOperation( operation ) => {
                return write!(f, "{}", format!("unknown grpc operator : {:?}", operation));
            },
            HibouCoreError::SolverUnknownSatisfiability => {
                return write!(f,  "{}", format!("solver returned Unknown"));
            }
        }
    }
}
