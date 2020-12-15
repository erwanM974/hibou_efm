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

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;

use crate::core::syntax::data::var_ref::VariableReference;
use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::builtin::bool::*;
use crate::core::syntax::data::builtin::number::*;
use crate::core::syntax::data::builtin::integer::TD_Integer;
use crate::core::syntax::data::builtin::float::TD_Float;
use crate::core::syntax::data::builtin::string::TD_String;

use crate::diversity::*;
use crate::diversity::expression::ExpressionAlt;

use crate::core::error::HibouCoreError;


fn symbol_reference_from_grpc(gen_ctx : &GeneralContext,
                              exe_ctx : &ExecutionContext,
                              symbol_fqn : &String,
                              expected_type : &TD_DataType) -> Result<TD_Generic,HibouCoreError> {
    match exe_ctx.get_sy_type_from_fqn( &symbol_fqn) {
        Err(e) => {
            return Err(e);
        },
        Ok( sy_type ) => {
            if &sy_type != expected_type {
                return Err( HibouCoreError::WronglyTypedGrpcInput(symbol_fqn.clone(), sy_type.clone(), expected_type.as_xlia_str() ));
            }
            match sy_type {
                TD_DataType::String => {
                    let td_str = TD_String::Reference( VariableReference::SYMBOL( exe_ctx.get_sy_id_from_fqn(&symbol_fqn).unwrap() ) );
                    return Ok( TD_Generic::String(td_str) );
                },
                TD_DataType::Integer => {
                    let td_int = TD_Integer::Reference( VariableReference::SYMBOL( exe_ctx.get_sy_id_from_fqn(&symbol_fqn).unwrap() ) );
                    return Ok( TD_Generic::Integer(td_int) );
                },
                TD_DataType::Float => {
                    let td_float = TD_Float::Reference( VariableReference::SYMBOL( exe_ctx.get_sy_id_from_fqn(&symbol_fqn).unwrap() ) );
                    return Ok( TD_Generic::Float(td_float) );
                },
                TD_DataType::Bool => {
                    let td_bool = TD_Bool::Reference( VariableReference::SYMBOL( exe_ctx.get_sy_id_from_fqn(&symbol_fqn).unwrap() ) );
                    return Ok( TD_Generic::Bool(td_bool) );
                }
            }
        }
    }
}

fn get_sub_expressions(gen_ctx : &GeneralContext,
                       exe_ctx : &ExecutionContext,
                       operation : &Operation,
                       expected_type : &TD_DataType) -> Result<Vec< TD_Generic >,HibouCoreError> {
    let mut sub_expressions : Vec< TD_Generic > = Vec::new();
    for expression in &operation.operand {
        match expression_from_grpc(gen_ctx,exe_ctx,expression,expected_type) {
            Err(e) => {
                return Err(e);
            },
            Ok( td_gen ) => {
                sub_expressions.push( td_gen );
            }
        }
    }
    return Ok( sub_expressions );
}

fn operation_from_grpc(gen_ctx : &GeneralContext,
                       exe_ctx : &ExecutionContext,
                       operation : &Operation,
                       expected_type : &TD_DataType) -> Result<TD_Generic,HibouCoreError> {
    if operation.operator_kind == ( OperatorKind::Add as i32 ) {
        match get_sub_expressions(gen_ctx,exe_ctx,operation,expected_type) {
            Err(e) => {
                return Err(e);
            },
            Ok( mut sub_expressions ) => {
                match expected_type {
                    TD_DataType::Integer => {
                        let mut add_vec : Vec<(ARITH_ADD_SIGN,TD_Integer)> = Vec::new();
                        for sub_expr in sub_expressions {
                            match sub_expr {
                                TD_Generic::Integer(td_int) => {
                                    add_vec.push( (ARITH_ADD_SIGN::Plus, td_int) );
                                },
                                _ => {
                                    return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                                }
                            }
                        }
                        return Ok( TD_Generic::Integer( TD_Integer::Add(add_vec) ) );
                    },
                    TD_DataType::Float => {
                        let mut add_vec : Vec<(ARITH_ADD_SIGN,TD_Float)> = Vec::new();
                        for sub_expr in sub_expressions {
                            match sub_expr {
                                TD_Generic::Float(td_float) => {
                                    add_vec.push( (ARITH_ADD_SIGN::Plus, td_float) );
                                },
                                _ => {
                                    return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                                }
                            }
                        }
                        return Ok( TD_Generic::Float( TD_Float::Add(add_vec) ) );
                    },
                    _ => {
                        return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                    }
                }
            }
        }
    } else if operation.operator_kind == ( OperatorKind::Minus as i32 ) {
        match get_sub_expressions(gen_ctx,exe_ctx,operation,expected_type) {
            Err(e) => {
                return Err(e);
            },
            Ok( mut sub_expressions ) => {
        match expected_type {
            TD_DataType::Integer => {
                let mut add_vec : Vec<(ARITH_ADD_SIGN,TD_Integer)> = Vec::new();
                let mut first : bool = true;
                for sub_expr in sub_expressions {
                    match sub_expr {
                        TD_Generic::Integer(td_int) => {
                            if first {
                                add_vec.push( (ARITH_ADD_SIGN::Plus, td_int) );
                                first = false;
                            } else {
                                add_vec.push( (ARITH_ADD_SIGN::Minus, td_int) );
                            }
                        },
                        _ => {
                            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                        }
                    }
                }
                return Ok( TD_Generic::Integer( TD_Integer::Add(add_vec) ) );
            },
            TD_DataType::Float => {
                let mut add_vec : Vec<(ARITH_ADD_SIGN,TD_Float)> = Vec::new();
                let mut first : bool = true;
                for sub_expr in sub_expressions {
                    match sub_expr {
                        TD_Generic::Float(td_float) => {
                            if first {
                                add_vec.push( (ARITH_ADD_SIGN::Plus, td_float) );
                                first = false;
                            } else {
                                add_vec.push( (ARITH_ADD_SIGN::Minus, td_float) );
                            }
                        },
                        _ => {
                            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                        }
                    }
                }
                return Ok( TD_Generic::Float( TD_Float::Add(add_vec) ) );
            },
            _ => {
                return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
            }
        }}}
    } else if operation.operator_kind == ( OperatorKind::Uminus as i32 ) {
        match get_sub_expressions(gen_ctx,exe_ctx,operation,expected_type) {
            Err(e) => {
                return Err(e);
            },
            Ok( mut sub_expressions ) => {
                match expected_type {
                    TD_DataType::Integer => {
                        if sub_expressions.len() != 1 {
                            return Err(HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()));
                        } else {
                            let td_gen = sub_expressions.remove(0);
                            match td_gen {
                                TD_Generic::Integer(td_int) => {
                                    return Ok(TD_Generic::Integer(TD_Integer::Minus(Box::new(td_int))));
                                },
                                _ => {
                                    return Err(HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()));
                                }
                            }
                        }
                    },
                    TD_DataType::Float => {
                        if sub_expressions.len() != 1 {
                            return Err(HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()));
                        } else {
                            let td_gen = sub_expressions.remove(0);
                            match td_gen {
                                TD_Generic::Float(td_float) => {
                                    return Ok(TD_Generic::Float(TD_Float::Minus(Box::new(td_float))));
                                },
                                _ => {
                                    return Err(HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()));
                                }
                            }
                        }
                    },
                    _ => {
                        return Err(HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()));
                    }
                }
            }}
    } else if operation.operator_kind == ( OperatorKind::Mult as i32 ) {
        match get_sub_expressions(gen_ctx,exe_ctx,operation,expected_type) {
            Err(e) => {
                return Err(e);
            },
            Ok( mut sub_expressions ) => {
        match expected_type {
            TD_DataType::Integer => {
                let mut factor_vec : Vec<(ARITH_FACTOR_SIGN,TD_Integer)> = Vec::new();
                for sub_expr in sub_expressions {
                    match sub_expr {
                        TD_Generic::Integer(td_int) => {
                            factor_vec.push( (ARITH_FACTOR_SIGN::Mult, td_int) );
                        },
                        _ => {
                            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                        }
                    }
                }
                return Ok( TD_Generic::Integer( TD_Integer::Factor(factor_vec) ) );
            },
            TD_DataType::Float => {
                let mut factor_vec : Vec<(ARITH_FACTOR_SIGN,TD_Float)> = Vec::new();
                for sub_expr in sub_expressions {
                    match sub_expr {
                        TD_Generic::Float(td_float) => {
                            factor_vec.push( (ARITH_FACTOR_SIGN::Mult, td_float) );
                        },
                        _ => {
                            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                        }
                    }
                }
                return Ok( TD_Generic::Float( TD_Float::Factor(factor_vec) ) );
            },
            _ => {
                return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
            }
        }}}
    } else if operation.operator_kind == ( OperatorKind::Div as i32 ) {
        match get_sub_expressions(gen_ctx,exe_ctx,operation,expected_type) {
            Err(e) => {
                return Err(e);
            },
            Ok( mut sub_expressions ) => {
        match expected_type {
            TD_DataType::Integer => {
                let mut factor_vec : Vec<(ARITH_FACTOR_SIGN,TD_Integer)> = Vec::new();
                let mut first : bool = true;
                for sub_expr in sub_expressions {
                    match sub_expr {
                        TD_Generic::Integer(td_int) => {
                            if first {
                                factor_vec.push( (ARITH_FACTOR_SIGN::Mult, td_int) );
                                first = false;
                            } else {
                                factor_vec.push( (ARITH_FACTOR_SIGN::Div, td_int) );
                            }
                        },
                        _ => {
                            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                        }
                    }
                }
                return Ok( TD_Generic::Integer( TD_Integer::Factor(factor_vec) ) );
            },
            TD_DataType::Float => {
                let mut factor_vec : Vec<(ARITH_FACTOR_SIGN,TD_Float)> = Vec::new();
                let mut first : bool = true;
                for sub_expr in sub_expressions {
                    match sub_expr {
                        TD_Generic::Float(td_float) => {
                            if first {
                                factor_vec.push( (ARITH_FACTOR_SIGN::Mult, td_float) );
                                first = false;
                            } else {
                                factor_vec.push( (ARITH_FACTOR_SIGN::Div, td_float) );
                            }
                        },
                        _ => {
                            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                        }
                    }
                }
                return Ok( TD_Generic::Float( TD_Float::Factor(factor_vec) ) );
            },
            _ => {
                return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
            }
        }}}
    } else if operation.operator_kind == ( OperatorKind::Or as i32 ) {
        match get_sub_expressions(gen_ctx,exe_ctx,operation,expected_type) {
            Err(e) => {
                return Err(e);
            },
            Ok( mut sub_expressions ) => {
        match expected_type {
            TD_DataType::Bool => {
                let mut sub_bool_vec : Vec<TD_Bool> = Vec::new();
                for sub_expr in sub_expressions {
                    match sub_expr {
                        TD_Generic::Bool(td_bool) => {
                            sub_bool_vec.push(td_bool);
                        },
                        _ => {
                            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                        }
                    }
                }
                return Ok( TD_Generic::Bool( TD_Bool::OR(sub_bool_vec) ) );
            },
            _ => {
                return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
            }
        }}}
    } else if operation.operator_kind == ( OperatorKind::And as i32 ) {
        match get_sub_expressions(gen_ctx,exe_ctx,operation,expected_type) {
            Err(e) => {
                return Err(e);
            },
            Ok( mut sub_expressions ) => {
        match expected_type {
            TD_DataType::Bool => {
                let mut sub_bool_vec : Vec<TD_Bool> = Vec::new();
                for sub_expr in sub_expressions {
                    match sub_expr {
                        TD_Generic::Bool(td_bool) => {
                            sub_bool_vec.push(td_bool);
                        },
                        _ => {
                            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                        }
                    }
                }
                return Ok( TD_Generic::Bool( TD_Bool::AND(sub_bool_vec) ) );
            },
            _ => {
                return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
            }
        }}}
    } else if operation.operator_kind == ( OperatorKind::Not as i32 ) {
        match get_sub_expressions(gen_ctx,exe_ctx,operation,expected_type) {
            Err(e) => {
                return Err(e);
            },
            Ok( mut sub_expressions ) => {
        match expected_type {
            TD_DataType::Bool => {
                if sub_expressions.len() != 1 {
                    return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                } else {
                    let sub_expr = sub_expressions.remove(0);
                    match sub_expr {
                        TD_Generic::Bool(td_bool) => {
                            return Ok( TD_Generic::Bool( TD_Bool::NOT(Box::new(td_bool)) ) );
                        },
                        _ => {
                            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
                        }
                    }
                }
            },
            _ => {
                return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
            }
        }}}
    } else if operation.operator_kind == ( OperatorKind::Eq as i32 ) {
        if operation.operand.len() != 2 || expected_type != &TD_DataType::Bool {
            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
        } else {
            match get_branches_of_comparaison_operator(gen_ctx,exe_ctx,operation) {
                Err(e) => {
                    return Err(e);
                },
                Ok( (got_type,mut sub_expressions) ) => {
                    let first =  sub_expressions.remove(0);
                    let second = sub_expressions.remove(0);
                    let td_bool = TD_Bool::COMPARE(Bool_Compare::Equal, Box::new(first), Box::new(second));
                    return Ok( TD_Generic::Bool( td_bool ) );
                }
            }
        }
    } else if operation.operator_kind == ( OperatorKind::Neq as i32 ) {
        if operation.operand.len() != 2 || expected_type != &TD_DataType::Bool {
            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
        } else {
            match get_branches_of_comparaison_operator(gen_ctx,exe_ctx,operation) {
                Err(e) => {
                    return Err(e);
                },
                Ok( (got_type,mut sub_expressions) ) => {
                    let first =  sub_expressions.remove(0);
                    let second = sub_expressions.remove(0);
                    let td_bool = TD_Bool::COMPARE(Bool_Compare::Different, Box::new(first), Box::new(second));
                    return Ok( TD_Generic::Bool( td_bool ) );
                }
            }
        }
    } else if operation.operator_kind == ( OperatorKind::Gt as i32 ) {
        if operation.operand.len() != 2 || expected_type != &TD_DataType::Bool {
            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
        } else {
            match get_branches_of_comparaison_operator(gen_ctx,exe_ctx,operation) {
                Err(e) => {
                    return Err(e);
                },
                Ok( (got_type,mut sub_expressions) ) => {
                    let first =  sub_expressions.remove(0);
                    let second = sub_expressions.remove(0);
                    let td_bool = TD_Bool::COMPARE(Bool_Compare::Greater, Box::new(first), Box::new(second));
                    return Ok( TD_Generic::Bool( td_bool ) );
                }
            }
        }
    } else if operation.operator_kind == ( OperatorKind::Gte as i32 ) {
        if operation.operand.len() != 2 || expected_type != &TD_DataType::Bool {
            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
        } else {
            match get_branches_of_comparaison_operator(gen_ctx,exe_ctx,operation) {
                Err(e) => {
                    return Err(e);
                },
                Ok( (got_type,mut sub_expressions) ) => {
                    let first =  sub_expressions.remove(0);
                    let second = sub_expressions.remove(0);
                    let td_bool = TD_Bool::COMPARE(Bool_Compare::GreaterOrEqual, Box::new(first), Box::new(second));
                    return Ok( TD_Generic::Bool( td_bool ) );
                }
            }
        }
    } else if operation.operator_kind == ( OperatorKind::Lt as i32 ) {
        if operation.operand.len() != 2 || expected_type != &TD_DataType::Bool {
            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
        } else {
            match get_branches_of_comparaison_operator(gen_ctx,exe_ctx,operation) {
                Err(e) => {
                    return Err(e);
                },
                Ok( (got_type,mut sub_expressions) ) => {
                    let first =  sub_expressions.remove(0);
                    let second = sub_expressions.remove(0);
                    let td_bool = TD_Bool::COMPARE(Bool_Compare::Lower, Box::new(first), Box::new(second));
                    return Ok( TD_Generic::Bool( td_bool ) );
                }
            }
        }
    } else if operation.operator_kind == ( OperatorKind::Lte as i32 ) {
        if operation.operand.len() != 2 || expected_type != &TD_DataType::Bool {
            return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) );
        } else {
            match get_branches_of_comparaison_operator(gen_ctx,exe_ctx,operation) {
                Err(e) => {
                    return Err(e);
                },
                Ok( (got_type,mut sub_expressions) ) => {
                    let first =  sub_expressions.remove(0);
                    let second = sub_expressions.remove(0);
                    let td_bool = TD_Bool::COMPARE(Bool_Compare::LowerOrEqual, Box::new(first), Box::new(second));
                    return Ok( TD_Generic::Bool( td_bool ) );
                }
            }
        }
    } else {
        return Err( HibouCoreError::UnknownOperatorInGrpcInputOperation(operation.clone()) );
    }
}

fn get_branches_of_comparaison_operator(gen_ctx : &GeneralContext,
                                        exe_ctx : &ExecutionContext,
                                        operation : &Operation) -> Result<(TD_DataType,Vec< TD_Generic >),HibouCoreError> {

    match get_sub_expressions(gen_ctx,exe_ctx,operation,&TD_DataType::String) {
        Err(e) => {}
        Ok( sub_expressions ) => {
            return Ok( (TD_DataType::String, sub_expressions) );
        }
    }

    match get_sub_expressions(gen_ctx,exe_ctx,operation,&TD_DataType::Bool) {
        Err(e) => {}
        Ok( sub_expressions ) => {
            return Ok( (TD_DataType::Bool, sub_expressions) );
        }
    }

    match get_sub_expressions(gen_ctx,exe_ctx,operation,&TD_DataType::Integer) {
        Err(e) => {}
        Ok( sub_expressions ) => {
            return Ok( (TD_DataType::Integer, sub_expressions) );
        }
    }

    match get_sub_expressions(gen_ctx,exe_ctx,operation,&TD_DataType::Float) {
        Err(e) => {}
        Ok( sub_expressions ) => {
            return Ok( (TD_DataType::Float, sub_expressions) );
        }
    }

    return Err( HibouCoreError::WronglyTypedGrpcInputOperation(operation.clone()) )
}



pub fn expression_from_grpc(gen_ctx : &GeneralContext,
                            exe_ctx : &ExecutionContext,
                            expression : &Expression,
                            expected_type : &TD_DataType ) -> Result<TD_Generic,HibouCoreError> {
    match &expression.expression_alt {
        None => {
            panic!("forbidden empty expression");
        },
        Some( expression_alt ) => {
            match expression_alt {
                ExpressionAlt::SymbolId( symbol_fqn ) => {
                    return symbol_reference_from_grpc(gen_ctx,exe_ctx,symbol_fqn,expected_type);
                },
                ExpressionAlt::VariableId( variable_fqn ) => {
                    //DIVERSITY should only return terms of symbols; no variables should appear in those terms
                    panic!();
                    //return variable_reference_from_grpc(gen_ctx,exe_ctx,var_id);
                },
                ExpressionAlt::Operation( sub_operation ) => {
                    return operation_from_grpc(gen_ctx,exe_ctx,sub_operation,expected_type);
                },
                ExpressionAlt::RawString( raw_string ) => {
                    if expected_type == &TD_DataType::String {
                        return Ok( TD_Generic::String(TD_String::Value(raw_string.clone())) );
                    } else {
                        return Err( HibouCoreError::WronglyTypedGrpcInput(raw_string.clone(), expected_type.clone(), TD_DataType::String.as_xlia_str() ) );
                    }
                },
                ExpressionAlt::RawInteger( raw_int ) => {
                    if expected_type == &TD_DataType::Integer {
                        return Ok( TD_Generic::Integer(TD_Integer::Value( *raw_int as i64 )) );
                    } else if expected_type == &TD_DataType::Float {
                        return Ok( TD_Generic::Float(TD_Float::Value( *raw_int as f64 )) );
                    } else {
                        return Err( HibouCoreError::WronglyTypedGrpcInput(raw_int.to_string(), expected_type.clone(), format!("{} or {}",TD_DataType::Integer.as_xlia_str(),TD_DataType::Float.as_xlia_str()) ) );
                    }
                },
                ExpressionAlt::RawFloat( raw_float ) => {
                    if expected_type == &TD_DataType::Integer {
                        if (*raw_float).fract() != 0.0 {
                            return Err( HibouCoreError::WronglyTypedGrpcInput(raw_float.to_string(), expected_type.clone(), format!("{} or {}",TD_DataType::Integer.as_xlia_str(),TD_DataType::Float.as_xlia_str()) ) );
                        } else {
                            return Ok( TD_Generic::Integer(TD_Integer::Value( *raw_float as i64 )) );
                        }
                    } else if expected_type == &TD_DataType::Float {
                        return Ok( TD_Generic::Float(TD_Float::Value( *raw_float as f64 )) );
                    } else {
                        return Err( HibouCoreError::WronglyTypedGrpcInput(raw_float.to_string(), expected_type.clone(), format!("{} or {}",TD_DataType::Integer.as_xlia_str(),TD_DataType::Float.as_xlia_str()) ) );
                    }
                },
                ExpressionAlt::RawBool( raw_bool ) => {
                    if expected_type == &TD_DataType::Bool {
                        let td_gen : TD_Generic;
                        if *raw_bool {
                            td_gen = TD_Generic::Bool(TD_Bool::TRUE);
                        } else {
                            td_gen = TD_Generic::Bool(TD_Bool::FALSE);
                        }
                        return Ok( td_gen );
                    } else {
                        return Err( HibouCoreError::WronglyTypedGrpcInput(raw_bool.to_string(), expected_type.clone(), TD_DataType::Bool.as_xlia_str() ) );
                    }
                }
            }
        }
    }
}