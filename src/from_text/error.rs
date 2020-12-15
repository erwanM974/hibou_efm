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

use crate::core::syntax::data::td_type::TD_DataType;

#[derive(Debug)]
pub enum HibouParsingError {
    FileFormatError(String,String),
    FileError(String),
    MatchError(String),
    // ***
    MissingMessageDeclarationError(String),
    MissingLifelineDeclarationError(String),
    MissingVariableDeclarationError(String),
    // ***
    ParameterUsageError(String),
    // ***
    MalformedArithmeticExpression(String),
    MalformedLogicExpression(String),
    ClockMisuse(String),
    // ***
    UnknownMessageParameter(String,usize),
    WrongMessageParameterType(TD_DataType,TD_DataType,String,String),
    WrongMessageParametersNumber(usize,usize,String),
    TimedTraceAbsentDelay(String),
    // ***
    NonDisjointTraceComponents,
    HsfSetupError(String)
}

impl fmt::Display for HibouParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HibouParsingError::FileFormatError( got, expected) => {
                return write!(f, "{}", format!("expected '.{}' file and got '.{}' file", expected, got));
            },
            HibouParsingError::FileError(sub_e) => {
                return write!(f, "{}", format!("error while reading SD conf file : {:}", sub_e));
            },
            HibouParsingError::MatchError(sub_e) => {
                return write!(f, "{}", format!("error while parsing SD string : {:}", sub_e));
            },
            HibouParsingError::MissingMessageDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing message declaration : {:}", sub_e));
            },
            HibouParsingError::MissingLifelineDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing lifeline declaration : {:}", sub_e));
            },
            HibouParsingError::MissingVariableDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing variable declaration : {:}", sub_e));
            },
            HibouParsingError::ParameterUsageError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; wrong parameter usage : {:}", sub_e));
            },
            HibouParsingError::MalformedArithmeticExpression(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; malformed arithmetic expression : {:}", sub_e));
            },
            HibouParsingError::MalformedLogicExpression(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; malformed logic expression : {:}", sub_e));
            },
            HibouParsingError::ClockMisuse(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; clock misuse : {:}", sub_e));
            },
            HibouParsingError::UnknownMessageParameter(ms_name,param_id) => {
                return write!(f, "{}", format!("error while parsing; unknown message parameter : message {} does not have parameter number {}", ms_name, param_id));
            },
            HibouParsingError::WrongMessageParameterType(expected, got, ms_name, ms_specs) => {
                return write!(f, "{}", format!("error while parsing trace; wrong message parameter type : expected {:?} - got {:?} - in message {:} - with specs {:}", expected, got, ms_name, ms_specs));
            },
            HibouParsingError::WrongMessageParametersNumber(expected, got, sub_e) => {
                return write!(f, "{}", format!("error while parsing trace; wrong message parameters numbers : expected {:?} - got {:?} - in message {:}", expected, got, sub_e));
            },
            HibouParsingError::TimedTraceAbsentDelay(sub_e) => {
                return write!(f, "{}", format!("error while parsing trace; timed trace absent delay : {:}", sub_e));
            },
            HibouParsingError::NonDisjointTraceComponents => {
                return write!(f, "{}", format!("error while parsing ; non disjoint trace canals"));
            },
            HibouParsingError::HsfSetupError(sub_e) => {
                return write!(f, "{}", format!("error while parsing setup section of .hsf file : {:}", sub_e));
            }
        }
    }
}





