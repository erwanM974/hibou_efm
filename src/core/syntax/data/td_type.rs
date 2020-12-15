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

use crate::diversity;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TD_DataType {
    //Clock,
    Bool,
    String,
    Integer,
    Float
}

impl TD_DataType {

    pub fn from_grpc(grpc_data_type_id : i32) -> TD_DataType {
        if grpc_data_type_id == (diversity::DataType::Boolean as i32) {
            return TD_DataType::Bool;
        } else if grpc_data_type_id == (diversity::DataType::String as i32) {
            return TD_DataType::String;
        } else if grpc_data_type_id == (diversity::DataType::Integer as i32) {
            return TD_DataType::Integer;
        } else if grpc_data_type_id == (diversity::DataType::Float as i32) {
            return TD_DataType::Float;
        } else if grpc_data_type_id == (diversity::DataType::Rational as i32) {
            return TD_DataType::Float;
        } else {
            println!("{}", grpc_data_type_id);
            panic!();
        }
    }

    pub fn as_xlia_str(&self) -> String {
        match self {/*
            TD_DataType::Clock => {
                return "Real".to_string();
            },*/
            TD_DataType::Bool => {
                return "bool".to_string();
            },
            TD_DataType::String => {
                return "string".to_string();
            },
            TD_DataType::Integer => {
                return "int".to_string();
            },
            TD_DataType::Float => {
                return "float".to_string();
            },
        }
    }

    pub fn as_smtlib_str(&self) -> String {
        match self {/*
            TD_DataType::Clock => {
                return "Real".to_string();
            },*/
            TD_DataType::Bool => {
                return "Bool".to_string();
            },
            TD_DataType::String => {
                return "String".to_string();
            },
            TD_DataType::Integer => {
                return "Int".to_string();
            },
            TD_DataType::Float => {
                return "Real".to_string();
            },
        }
    }

}