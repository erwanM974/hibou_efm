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

use std::fmt::Write;

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

use crate::xlia::xlia_build_name_tools::*;

pub fn td_generic_to_xlia(gen_ctx : &GeneralContext,  td_gen : &TD_Generic) -> String {
    match td_gen {
        TD_Generic::Bool(td_bool) => {
            return td_bool_to_xlia(gen_ctx,  td_bool);
        },
        TD_Generic::String(td_string) => {
            return td_string_to_xlia(gen_ctx,  td_string);
        },
        TD_Generic::Integer(td_int) => {
            return td_int_to_xlia(gen_ctx,  td_int);
        },
        TD_Generic::Float(td_float) => {
            return td_float_to_xlia(gen_ctx,  td_float);
        }
    }
}

fn var_ref_to_xlia(gen_ctx : &GeneralContext,  var_ref : &VariableReference) -> String {
    match var_ref {
        VariableReference::VARIABLE( var_id ) => {
            return variable_diversity_name(gen_ctx,*var_id);
        },
        VariableReference::MSG_PARAMETER( ms_id, pr_id) => {
            return message_parameter_diversity_name(gen_ctx,*ms_id,*pr_id);
        },
        _ => {
            panic!();
        }
    }
}



fn td_int_to_xlia(gen_ctx : &GeneralContext,  td_int : &TD_Integer) -> String {
    match td_int {
        TD_Integer::Value( value ) => {
            return value.to_string();
        },
        TD_Integer::Reference(var_ref) => {
            return var_ref_to_xlia(gen_ctx,var_ref);
        },
        TD_Integer::Minus( minused ) => {
            let minused_td_int = td_int_to_xlia(gen_ctx,&*minused);
            return format!("- {}",minused_td_int);
        }
        TD_Integer::Add( adds ) => {
            let mut operand = String::new();
            let mut first : bool = true;
            for (add_sign,sub_int) in adds {
                let sub_int_expr = td_int_to_xlia(gen_ctx,sub_int);
                match add_sign {
                    ARITH_ADD_SIGN::Plus => {
                        if first {
                            operand.push_str( &sub_int_expr );
                            first = false;
                        } else {
                            operand.push_str( &format!("+ {}",sub_int_expr) );
                        }
                    },
                    ARITH_ADD_SIGN::Minus => {
                        if first {
                            first = false;
                        }
                        operand.push_str( &format!("- {}",sub_int_expr) );
                    }
                }
            }
            return operand;
        },
        TD_Integer::Factor( factor ) => {
            let mut operand = String::new();
            let mut first : bool = true;
            for (add_sign,sub_int) in factor {
                let sub_int_expr = td_int_to_xlia(gen_ctx,sub_int);
                match add_sign {
                    ARITH_FACTOR_SIGN::Mult => {
                        if first {
                            operand.push_str( &sub_int_expr );
                            first = false;
                        } else {
                            operand.push_str( &format!("* {}",sub_int_expr) );
                        }
                    },
                    ARITH_FACTOR_SIGN::Div => {
                        if first {
                            first = false;
                            operand.push_str( &format!("(1/{})",sub_int_expr) );
                        } else {
                            operand.push_str( &format!("* (1/{})",sub_int_expr) );
                        }
                    }
                }
            }
            return operand;
        }
    }
}

fn print_float(my_float : f64) -> String {
    let left_str = my_float.to_string();
    if left_str.contains(".") {
        return left_str;
    } else {
        return format!("{}.0",&left_str);
    }
}

fn td_float_to_xlia(gen_ctx : &GeneralContext,  td_float : &TD_Float) -> String {
    match td_float {
        TD_Float::Value( value ) => {
            return print_float(*value);
        },
        TD_Float::Reference(var_ref) => {
            return var_ref_to_xlia(gen_ctx,var_ref);
        },
        TD_Float::Minus( minused ) => {
            let minused_td_int = td_float_to_xlia(gen_ctx,&*minused);
            return format!("- {}",minused_td_int);
        }
        TD_Float::Add( adds ) => {
            let mut operand = String::new();
            let mut first : bool = true;
            for (add_sign,sub_int) in adds {
                let sub_int_expr = td_float_to_xlia(gen_ctx,sub_int);
                match add_sign {
                    ARITH_ADD_SIGN::Plus => {
                        if first {
                            operand.push_str( &sub_int_expr );
                            first = false;
                        } else {
                            operand.push_str( &format!("+ {}",sub_int_expr) );
                        }
                    },
                    ARITH_ADD_SIGN::Minus => {
                        if first {
                            first = false;
                        }
                        operand.push_str( &format!("- {}",sub_int_expr) );
                    }
                }
            }
            return operand;
        },
        TD_Float::Factor( factor ) => {
            let mut operand = String::new();
            let mut first : bool = true;
            for (add_sign,sub_int) in factor {
                let sub_int_expr = td_float_to_xlia(gen_ctx,sub_int);
                match add_sign {
                    ARITH_FACTOR_SIGN::Mult => {
                        if first {
                            operand.push_str( &sub_int_expr );
                            first = false;
                        } else {
                            operand.push_str( &format!("* {}",sub_int_expr) );
                        }
                    },
                    ARITH_FACTOR_SIGN::Div => {
                        if first {
                            first = false;
                            operand.push_str( &format!("(1.0/{})",sub_int_expr) );
                        } else {
                            operand.push_str( &format!("* (1.0/{})",sub_int_expr) );
                        }
                    }
                }
            }
            return operand;
        }
    }
}

fn td_string_to_xlia(gen_ctx : &GeneralContext,  td_string : &TD_String) -> String {
    match td_string {
        TD_String::Reference(var_ref) => {
            return var_ref_to_xlia(gen_ctx,var_ref);
        },
        TD_String::Value( value ) => {
            return format!("\"{}\"",value);
        }
    }
}


pub fn td_bool_to_xlia(gen_ctx : &GeneralContext,  td_bool : &TD_Bool) -> String {
    match td_bool {
        TD_Bool::TRUE => {
            return "true".to_string();
        },
        TD_Bool::FALSE => {
            return "false".to_string();
        },
        TD_Bool::AND(sub_bools) => {
            let mut and_string = String::new();
            let mut first : bool = true;
            for sub_bool in sub_bools {
                let sub_string = td_bool_to_xlia(gen_ctx,sub_bool);
                if first {
                    first = false;
                    and_string.push_str(&sub_string);
                } else {
                    and_string.push_str(" && ");
                    and_string.push_str(&sub_string);
                }
            }
            return and_string;
            /*
            let mut sub_exprs : Vec<String> = Vec::new();
            for sub_bool in sub_bools {
                sub_exprs.push( td_bool_to_xlia(gen_ctx,sub_bool) );
            }
            return sub_exprs.iter().fold(String::new(),|mut s,n| {write!(s," && {}",n).ok(); s});*/
        },
        TD_Bool::OR(sub_bools) => {
            /*
            let mut sub_exprs : Vec<String> = Vec::new();
            for sub_bool in sub_bools {
                sub_exprs.push( td_bool_to_xlia(gen_ctx,sub_bool) );
            }
            return sub_exprs.iter().fold(String::new(),|mut s,n| {write!(s," || {}",n).ok(); s});*/
            let mut and_string = String::new();
            let mut first : bool = true;
            for sub_bool in sub_bools {
                let sub_string = td_bool_to_xlia(gen_ctx,sub_bool);
                if first {
                    first = false;
                    and_string.push_str(&sub_string);
                } else {
                    and_string.push_str(" || ");
                    and_string.push_str(&sub_string);
                }
            }
            return and_string;
        },
        TD_Bool::NOT(sub_bool) => {
            return format!("not {}", td_bool_to_xlia(gen_ctx,sub_bool));
        },
        TD_Bool::COMPARE(bool_compare,first,second) => {
            let first_expression = td_generic_to_xlia(gen_ctx,&*first);
            let second_expression = td_generic_to_xlia(gen_ctx,&*second);
            match bool_compare {
                Bool_Compare::Equal => {
                    return format!("{} == {}", first_expression, second_expression);
                },
                Bool_Compare::Different => {
                    return format!("{} != {}", first_expression, second_expression);
                },
                Bool_Compare::Greater => {
                    return format!("{} > {}", first_expression, second_expression);
                },
                Bool_Compare::GreaterOrEqual => {
                    return format!("{} >= {}", first_expression, second_expression);
                },
                Bool_Compare::Lower => {
                    return format!("{} < {}", first_expression, second_expression);
                },
                Bool_Compare::LowerOrEqual => {
                    return format!("{} <= {}", first_expression, second_expression);
                }
            }
        },
        TD_Bool::Reference( var_ref ) => {
            return var_ref_to_xlia(gen_ctx,var_ref);
        }
    }
}