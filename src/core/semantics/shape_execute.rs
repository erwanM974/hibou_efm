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

use std::collections::{HashSet,HashMap};



use crate::core::syntax::position::*;
use crate::core::syntax::action::*;
use crate::core::syntax::interaction::*;

use crate::core::semantics::frontier::*;
use crate::core::semantics::prune::*;
use crate::core::semantics::varmap::*;

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;

use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::builtin::bool::TD_Bool;

use crate::core::trace::*;
use crate::core::error::HibouCoreError;

pub fn shape_execute( gen_ctx : &GeneralContext,
                exe_ctx : &mut ExecutionContext,
                my_int : &Interaction,
                my_pos : &Position)
                -> Result< (Interaction,Position,ObservableAction,bool) , HibouCoreError > { // if events_to_match is None then we will use the usize index 0 for the execution result unconstrained by a trace action
    // ***
    let model_action = my_int.get_sub_interaction(my_pos).as_leaf();
    // ***
    return shape_execute_rec(gen_ctx,exe_ctx, my_int, &model_action.lf_act.lf_id, my_pos, &mut Vec::new());
}





fn shape_execute_left_in_schedule_operator(   gen_ctx : &GeneralContext,
                                        exe_ctx : &mut ExecutionContext,
                                        my_i1 : &Interaction,
                                        my_i2 : &Interaction,
                                        concerned_lf : &usize,
                                        my_sub_pos : &Position,
                                        op_kind : &ScheduleOperatorKind,
                                        current_position : &mut Vec<u32>) -> Result< (Interaction,Position,ObservableAction,bool) , HibouCoreError > {
    current_position.push(1);
    match shape_execute_rec(gen_ctx,exe_ctx, my_i1,concerned_lf,my_sub_pos, current_position) {
        Err(e) => {return Err(e);}
        Ok( (new_i1,final_pos,final_action,needs_scoping) ) => {
            let boxed_i2 = Box::new( my_i2.clone() );
            let final_interaction : Interaction;
            match op_kind {
                ScheduleOperatorKind::Strict => {
                    final_interaction = Interaction::Strict(Box::new(new_i1), boxed_i2);
                },
                ScheduleOperatorKind::Seq => {
                    final_interaction = Interaction::Seq(Box::new(new_i1), boxed_i2);
                },
                ScheduleOperatorKind::Par => {
                    final_interaction = Interaction::Par(Box::new(new_i1), boxed_i2);
                }
            }
            return Ok( (final_interaction,final_pos,final_action,needs_scoping) );
        }
    }
}



fn shape_execute_rec(gen_ctx : &GeneralContext,
                    exe_ctx : &mut ExecutionContext,
                    my_int : &Interaction,
                    concerned_lf : &usize,
                    target_pos : &Position,
                    current_position : &mut Vec<u32>) -> Result< (Interaction,Position,ObservableAction,bool) , HibouCoreError > {
    match target_pos {
        Position::Epsilon => {
            match my_int {
                &Interaction::Action(ref model_action) => {
                    return Ok( (my_int.clone(),Position::from_vec(current_position),model_action.clone(),false) );
                },
                _ => {
                    return Err( HibouCoreError::PositionError(my_int.clone(), target_pos.clone()) );
                }
            }
        },
        Position::Left(sub_pos) => {
            match my_int {
                &Interaction::Alt(ref i1,_) => {
                    return shape_execute_rec(gen_ctx,exe_ctx, &*i1,concerned_lf,&*sub_pos,current_position);
                },
                &Interaction::Loop(ref lkind, ref i1) => {
                    return shape_execute_left_in_schedule_operator(gen_ctx,exe_ctx,&*i1, my_int, concerned_lf,&*sub_pos, lkind,current_position);
                },
                &Interaction::Scope(ref scope, ref i1) => {
                    let new_i1 = exe_ctx.open_scope(gen_ctx, scope, i1);
                    match shape_execute_rec(gen_ctx,exe_ctx, &new_i1, concerned_lf,&*sub_pos,current_position) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( (final_interaction,final_pos,final_action,needs_scoping) ) => {
                            return Ok( (final_interaction,final_pos,final_action,true) );
                        }
                    }
                },
                &Interaction::Strict(ref i1,ref i2) => {
                    return shape_execute_left_in_schedule_operator(gen_ctx,exe_ctx,&*i1, &*i2, concerned_lf,&*sub_pos, &ScheduleOperatorKind::Strict,current_position);
                },
                &Interaction::Seq(ref i1,ref i2) => {
                    return shape_execute_left_in_schedule_operator(gen_ctx,exe_ctx,&*i1, &*i2, concerned_lf,&*sub_pos, &ScheduleOperatorKind::Seq,current_position);
                },
                &Interaction::Par(ref i1,ref i2) => {
                    return shape_execute_left_in_schedule_operator(gen_ctx,exe_ctx,&*i1, &*i2, concerned_lf,&*sub_pos, &ScheduleOperatorKind::Par,current_position);
                },
                _ => {
                    return Err( HibouCoreError::PositionError(my_int.clone(), target_pos.clone()) );
                }
            }
        },
        Position::Right(sub_pos) => {
            match my_int {
                &Interaction::Alt(_,ref i2) => {
                    return shape_execute_rec(gen_ctx,exe_ctx, &*i2,concerned_lf,&*sub_pos,current_position);
                },
                &Interaction::Strict(_,ref i2) => {
                    return shape_execute_rec(gen_ctx,exe_ctx, &*i2,concerned_lf,&*sub_pos,current_position);
                },
                &Interaction::Seq(ref i1,ref i2) => {
                    let new_i1 = prune(i1,*concerned_lf);
                    // ***
                    if new_i1 == Interaction::Empty {
                        match shape_execute_rec(gen_ctx,exe_ctx,&*i2, concerned_lf,&*sub_pos,current_position) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( (new_i2,final_pos,final_action,needs_scoping) ) => {
                                return Ok( (new_i2,final_pos,final_action,needs_scoping) );
                            }
                        }
                    } else {
                        current_position.push(2);
                        match shape_execute_rec(gen_ctx,exe_ctx,&*i2, concerned_lf,&*sub_pos,current_position) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( (new_i2,final_pos,final_action,needs_scoping) ) => {
                                let final_interaction = Interaction::Seq(Box::new(new_i1), Box::new(new_i2) );
                                return Ok( (final_interaction,final_pos,final_action,needs_scoping) );
                            }
                        }
                    }
                },
                &Interaction::Par(ref i1,ref i2) => {
                    current_position.push(2);
                    match shape_execute_rec(gen_ctx,exe_ctx,&*i2, concerned_lf,&*sub_pos,current_position) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( (new_i2,final_pos,final_action,needs_scoping) ) => {
                            let final_interaction = Interaction::Par(i1.clone(), Box::new(new_i2) );
                            return Ok( (final_interaction,final_pos,final_action,needs_scoping) );
                        }
                    }
                },
                _ => {
                    return Err( HibouCoreError::PositionError(my_int.clone(), target_pos.clone()) );
                }
            }
        }
    }
}