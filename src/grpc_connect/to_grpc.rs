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

use crate::grpc_connect::xlia_reference_name_tools::*;

pub fn td_generic_to_grpc(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, td_gen : &TD_Generic) -> Expression {
    match td_gen {
        TD_Generic::Bool(td_bool) => {
            return td_bool_to_grpc(gen_ctx, exe_ctx, lf_id,td_bool);
        },
        TD_Generic::String(td_string) => {
            return td_string_to_grpc(gen_ctx, exe_ctx, lf_id,td_string);
        },
        TD_Generic::Integer(td_int) => {
            return td_int_to_grpc(gen_ctx, exe_ctx, lf_id,td_int);
        },
        TD_Generic::Float(td_float) => {
            return td_float_to_grpc(gen_ctx, exe_ctx, lf_id,td_float);
        }
    }
}

fn var_ref_to_grpc(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, var_ref : &VariableReference) -> Expression {
    match var_ref {
        VariableReference::VARIABLE( var_id ) => {
            let (_,var_child_idx) = exe_ctx.get_vr_parent_name_and_child_id(gen_ctx, *var_id).unwrap();
            let fqn = variable_diversity_fqn(gen_ctx,exe_ctx,lf_id,*var_id);
            let expr_kind = ExpressionAlt::VariableId(fqn);
            return Expression{expression_alt:Some(expr_kind)};
        },
        VariableReference::SYMBOL( symb_id ) => {
            let symbol_diversity_name = exe_ctx.get_sy_diversity_name( *symb_id).unwrap();
            let expr_kind = ExpressionAlt::SymbolId(symbol_diversity_name);
            return Expression{expression_alt:Some(expr_kind)};
        },
        _ => {
            panic!();
        }
    }
}



fn td_int_to_grpc(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, td_int : &TD_Integer) -> Expression {
    match td_int {
        TD_Integer::Value( value ) => {
            let expr_kind = ExpressionAlt::RawInteger( *value );
            return Expression{expression_alt:Some(expr_kind)};
        },
        TD_Integer::Reference(var_ref) => {
            return var_ref_to_grpc(gen_ctx,exe_ctx,lf_id,var_ref);
        },
        TD_Integer::Minus( minused ) => {
            let minused_td_int = td_int_to_grpc(gen_ctx,exe_ctx,lf_id,&*minused);
            let minus_op = ExpressionAlt::Operation(Operation{operator_kind:OperatorKind::Uminus as i32,operand:vec![minused_td_int]});
            let minus_sub_expr = Expression{expression_alt:Some( minus_op )};
            return minus_sub_expr;
        }
        TD_Integer::Add( adds ) => {
            let mut operand : Vec<Expression> = Vec::new();
            for (add_sign,sub_int) in adds {
                let sub_int_expr = td_int_to_grpc(gen_ctx,exe_ctx,lf_id,sub_int);
                match add_sign {
                    ARITH_ADD_SIGN::Plus => {
                        operand.push(sub_int_expr);
                    },
                    ARITH_ADD_SIGN::Minus => {
                        let minus_op = ExpressionAlt::Operation(Operation{operator_kind:OperatorKind::Uminus as i32,operand:vec![sub_int_expr]});
                        let minus_sub_expr = Expression{expression_alt:Some( minus_op )};
                        operand.push(minus_sub_expr);
                    }
                }
            }
            let ex_alt = ExpressionAlt::Operation( Operation{operator_kind: (OperatorKind::Add as i32) , operand} );
            return Expression{expression_alt:Some(ex_alt)};
        },
        TD_Integer::Factor( factor ) => {
            let mut operand : Vec<Expression> = Vec::new();
            for (fact_sign,sub_int) in factor {
                let sub_int_expr = td_int_to_grpc(gen_ctx,exe_ctx,lf_id,sub_int);
                match fact_sign {
                    ARITH_FACTOR_SIGN::Mult => {
                        operand.push(sub_int_expr);
                    },
                    ARITH_FACTOR_SIGN::Div => {
                        let integer_one = Expression{expression_alt:Some( ExpressionAlt::RawInteger(1 as i64) )};
                        let division_op = ExpressionAlt::Operation(Operation{operator_kind:OperatorKind::Div as i32,operand:vec![integer_one,sub_int_expr]});
                        let division_sub_expr = Expression{expression_alt:Some( division_op )};
                        operand.push(division_sub_expr);
                    }
                }
            }
            let ex_alt = ExpressionAlt::Operation( Operation{operator_kind: (OperatorKind::Mult as i32) , operand} );
            return Expression{expression_alt:Some(ex_alt)};
        }
    }
}

fn td_float_to_grpc(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, td_float : &TD_Float) -> Expression {
    match td_float {
        TD_Float::Value( value ) => {
            let expr_kind = ExpressionAlt::RawFloat( *value );
            return Expression{expression_alt:Some(expr_kind)};
        },
        TD_Float::Reference(var_ref) => {
            return var_ref_to_grpc(gen_ctx,exe_ctx,lf_id,var_ref);
        },
        TD_Float::Minus( minused ) => {
            let minused_td_int = td_float_to_grpc(gen_ctx,exe_ctx,lf_id,&*minused);
            let minus_op = ExpressionAlt::Operation(Operation{operator_kind:OperatorKind::Uminus as i32,operand:vec![minused_td_int]});
            let minus_sub_expr = Expression{expression_alt:Some( minus_op )};
            return minus_sub_expr;
        }
        TD_Float::Add( adds ) => {
            let mut operand : Vec<Expression> = Vec::new();
            for (add_sign,sub_int) in adds {
                let sub_int_expr = td_float_to_grpc(gen_ctx,exe_ctx,lf_id,sub_int);
                match add_sign {
                    ARITH_ADD_SIGN::Plus => {
                        operand.push(sub_int_expr);
                    },
                    ARITH_ADD_SIGN::Minus => {
                        let minus_op = ExpressionAlt::Operation(Operation{operator_kind:OperatorKind::Uminus as i32,operand:vec![sub_int_expr]});
                        let minus_sub_expr = Expression{expression_alt:Some( minus_op )};
                        operand.push(minus_sub_expr);
                    }
                }
            }
            let ex_alt = ExpressionAlt::Operation( Operation{operator_kind: (OperatorKind::Add as i32) , operand} );
            return Expression{expression_alt:Some(ex_alt)};
        },
        TD_Float::Factor( factor ) => {
            let mut operand : Vec<Expression> = Vec::new();
            for (fact_sign,sub_int) in factor {
                let sub_int_expr = td_float_to_grpc(gen_ctx,exe_ctx,lf_id,sub_int);
                match fact_sign {
                    ARITH_FACTOR_SIGN::Mult => {
                        operand.push(sub_int_expr);
                    },
                    ARITH_FACTOR_SIGN::Div => {
                        let float_one = Expression{expression_alt:Some( ExpressionAlt::RawFloat(1.0 as f64) )};
                        let division_op = ExpressionAlt::Operation(Operation{operator_kind:OperatorKind::Div as i32,operand:vec![float_one,sub_int_expr]});
                        let division_sub_expr = Expression{expression_alt:Some( division_op )};
                        operand.push(division_sub_expr);
                    }
                }
            }
            let ex_alt = ExpressionAlt::Operation( Operation{operator_kind: (OperatorKind::Mult as i32) , operand} );
            return Expression{expression_alt:Some(ex_alt)};
        }
    }
}

fn td_string_to_grpc(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, td_string : &TD_String) -> Expression {
    match td_string {
        TD_String::Reference(var_ref) => {
            return var_ref_to_grpc(gen_ctx,exe_ctx,lf_id,var_ref);
        },
        TD_String::Value( value ) => {
            let expr_kind = ExpressionAlt::RawString( value.clone() );
            return Expression{expression_alt:Some(expr_kind)};
        }
    }
}


pub fn td_bool_to_grpc(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, td_bool : &TD_Bool) -> Expression {
    match td_bool {
        TD_Bool::TRUE => {
            let expr_kind = ExpressionAlt::RawBool(true);
            return Expression{expression_alt:Some(expr_kind)};
        },
        TD_Bool::FALSE => {
            let expr_kind = ExpressionAlt::RawBool(false);
            return Expression{expression_alt:Some(expr_kind)};
        },
        TD_Bool::AND(sub_bools) => {
            let mut sub_exprs : Vec<Expression> = Vec::new();
            for sub_bool in sub_bools {
                sub_exprs.push( td_bool_to_grpc(gen_ctx,exe_ctx,lf_id,sub_bool) );
            }
            let operation = Operation{operator_kind: (OperatorKind::And as i32) , operand:sub_exprs};
            let expr_kind = ExpressionAlt::Operation(operation);
            return Expression{expression_alt:Some(expr_kind)};
        },
        TD_Bool::OR(sub_bools) => {
            let mut sub_exprs : Vec<Expression> = Vec::new();
            for sub_bool in sub_bools {
                sub_exprs.push( td_bool_to_grpc(gen_ctx,exe_ctx,lf_id,sub_bool) );
            }
            let operation = Operation{operator_kind: (OperatorKind::Or as i32) , operand:sub_exprs};
            let expr_kind = ExpressionAlt::Operation(operation);
            return Expression{expression_alt:Some(expr_kind)};
        },
        TD_Bool::NOT(sub_bool) => {
            let operation = Operation{operator_kind: (OperatorKind::Not as i32) , operand:vec![ td_bool_to_grpc(gen_ctx,exe_ctx,lf_id,&*sub_bool) ]};
            let expr_kind = ExpressionAlt::Operation(operation);
            return Expression{expression_alt:Some(expr_kind)};
        },
        TD_Bool::COMPARE(bool_compare,first,second) => {
            let first_expression = td_generic_to_grpc(gen_ctx,exe_ctx,lf_id,&*first);
            let second_expression = td_generic_to_grpc(gen_ctx,exe_ctx,lf_id,&*first);
            match bool_compare {
                Bool_Compare::Equal => {
                    let operation = Operation{operator_kind: (OperatorKind::Eq as i32) , operand:vec![ first_expression,second_expression ]};
                    let expr_kind = ExpressionAlt::Operation(operation);
                    return Expression{expression_alt:Some(expr_kind)};
                },
                Bool_Compare::Different => {
                    let operation = Operation{operator_kind: (OperatorKind::Neq as i32) , operand:vec![ first_expression,second_expression ]};
                    let expr_kind = ExpressionAlt::Operation(operation);
                    return Expression{expression_alt:Some(expr_kind)};
                },
                Bool_Compare::Greater => {
                    let operation = Operation{operator_kind: (OperatorKind::Gt as i32) , operand:vec![ first_expression,second_expression ]};
                    let expr_kind = ExpressionAlt::Operation(operation);
                    return Expression{expression_alt:Some(expr_kind)};
                },
                Bool_Compare::GreaterOrEqual => {
                    let operation = Operation{operator_kind: (OperatorKind::Gte as i32) , operand:vec![ first_expression,second_expression ]};
                    let expr_kind = ExpressionAlt::Operation(operation);
                    return Expression{expression_alt:Some(expr_kind)};
                },
                Bool_Compare::Lower => {
                    let operation = Operation{operator_kind: (OperatorKind::Lt as i32) , operand:vec![ first_expression,second_expression ]};
                    let expr_kind = ExpressionAlt::Operation(operation);
                    return Expression{expression_alt:Some(expr_kind)};
                },
                Bool_Compare::LowerOrEqual => {
                    let operation = Operation{operator_kind: (OperatorKind::Lte as i32) , operand:vec![ first_expression,second_expression ]};
                    let expr_kind = ExpressionAlt::Operation(operation);
                    return Expression{expression_alt:Some(expr_kind)};
                }
            }
        },
        TD_Bool::Reference( var_ref ) => {
            return var_ref_to_grpc(gen_ctx,exe_ctx,lf_id,var_ref);
        }
    }
}