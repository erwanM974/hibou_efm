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
use std::collections::btree_map::BTreeMap;

// ***

use crate::core::context::general::GeneralContext;
use crate::core::syntax::action::*;
use crate::core::syntax::interaction::*;
use crate::core::trace::{TraceAction,TraceActionKind};
use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::builtin::bool::*;
use crate::core::syntax::data::builtin::integer::*;
use crate::core::syntax::data::builtin::float::*;
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::builtin::string::*;
use crate::core::syntax::data::var_ref::VariableReference;

use crate::core::semantics::varmap::VarMapAble;

use crate::core::error::HibouCoreError;


#[derive(Clone, PartialEq, Debug)]
pub struct ExecutionContext {
    symbol_counter : usize,
    symbol_types : BTreeMap<usize,TD_DataType>,
    symbol_diversity_names : BTreeMap<usize,String>,
    diversity_symbols_reverse_map : BTreeMap<String,usize>,
    // ********** ********** ********** ********** ********** ********** **********
    vr_id_counter : usize,
    vr_originals : BTreeMap<usize,(usize,u32)>,    // key is the unique ID
                                                   // first arg is the vr_id of the parent variable of which the current is an instance
                                                   // second arg is the instance number
    vr_instances_count : BTreeMap<usize,u32>,
    active_clocks : HashSet<usize>,
    // ********** ********** ********** ********** ********** ********** **********
    interpretation : BTreeMap<usize, BTreeMap<usize,TD_Generic> >,  // key is the lifeline
                                                                    // arg is mapping to current value
    path_condition : TD_Bool
}

impl ExecutionContext {

    pub fn get_path_condition(&self) -> &TD_Bool {
        return &self.path_condition;
    }

    pub fn set_path_condition(&mut self, new_path_condition : TD_Bool) {
        self.path_condition = new_path_condition;
    }

    pub fn get_lf_interpretation(&self, lf_id : usize) -> Option< &BTreeMap<usize,TD_Generic> > {
        return self.interpretation.get(&lf_id);
    }

    pub fn set_lf_interpretation(&mut self, lf_id : usize, new_lf_interpretation : BTreeMap<usize,TD_Generic>) {
        self.interpretation.insert( lf_id, new_lf_interpretation);
    }

    pub fn new(gen_ctx : &GeneralContext,interpretation:BTreeMap<usize, BTreeMap<usize,TD_Generic> >,symbol_counter:usize) -> ExecutionContext {
        return ExecutionContext{
            symbol_counter:symbol_counter,
            symbol_types:BTreeMap::new(),
            symbol_diversity_names:BTreeMap::new(),
            diversity_symbols_reverse_map:BTreeMap::new(),
            vr_id_counter:gen_ctx.get_vr_num(),
            vr_originals:BTreeMap::new(),
            vr_instances_count:BTreeMap::new(),
            active_clocks : gen_ctx.get_clocks().clone(),
            interpretation:interpretation,
            path_condition:TD_Bool::TRUE
        }
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********



    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_vr_num(&self) -> usize {
        return self.vr_id_counter;
    }

    pub fn get_active_clocks(&self) -> &HashSet<usize> {
        return &self.active_clocks;
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_name(&self, gen_ctx : &GeneralContext, lf_id : usize) -> Result<String,HibouCoreError> {
        return gen_ctx.get_lf_name(lf_id);
    }

    pub fn get_ms_name(&self, gen_ctx : &GeneralContext, ms_id : usize) -> Result<String,HibouCoreError> {
        return gen_ctx.get_ms_name(ms_id);
    }

    pub fn get_vr_parent_name_and_child_id(&self, gen_ctx: &GeneralContext, vr_id: usize) -> Result<(String, u32), HibouCoreError> {
        if vr_id < gen_ctx.get_vr_num() {
            return Ok((gen_ctx.get_vr_name(vr_id).unwrap(), 0));
        } else {
            match self.vr_originals.get(&vr_id) {
                None => {
                    return Err(HibouCoreError::UnknownParameter(vr_id));
                }
                Some((parent_vr_id, instance_number)) => {
                    let parent_vr_name = gen_ctx.get_vr_name(*parent_vr_id).unwrap();
                    return Ok((parent_vr_name, *instance_number));
                }
            }
        }
    }

    pub fn get_vr_name(&self, gen_ctx : &GeneralContext, vr_id : usize) -> Result<String,HibouCoreError> {
        if vr_id < gen_ctx.get_vr_num() {
            return gen_ctx.get_vr_name(vr_id);
        } else {
            match self.vr_originals.get(&vr_id ) {
                None => {
                    return Err( HibouCoreError::UnknownParameter(vr_id) );
                }
                Some( (parent_vr_id,instance_number) ) => {
                    let parent_vr_name = gen_ctx.get_vr_name(*parent_vr_id).unwrap();
                    return Ok( format!("{}_{:}", parent_vr_name, instance_number) );
                }
            }
        }
    }

    pub fn get_vr_type(&self, gen_ctx: &GeneralContext, vr_id: usize) -> Result<TD_DataType, HibouCoreError> {
        if vr_id < gen_ctx.get_vr_num() {
            return gen_ctx.get_vr_type(vr_id);
        } else {
            match self.vr_originals.get(&vr_id) {
                None => {
                    return Err(HibouCoreError::UnknownParameter(vr_id));
                }
                Some((parent_vr_id, instance_number)) => {
                    let parent_vr_type = gen_ctx.get_vr_type(*parent_vr_id).unwrap();
                    return Ok(parent_vr_type);
                }
            }
        }
    }

    pub fn is_clock(&self, gen_ctx: &GeneralContext, vr_id: usize) -> Result<bool, HibouCoreError> {
        if vr_id < gen_ctx.get_vr_num() {
            return Ok( gen_ctx.is_clock(vr_id) );
        } else {
            match self.vr_originals.get(&vr_id) {
                None => {
                    return Err(HibouCoreError::UnknownParameter(vr_id));
                }
                Some((parent_vr_id, instance_number)) => {
                    let parent_is_clock = gen_ctx.is_clock(*parent_vr_id);
                    return Ok( parent_is_clock );
                }
            }
        }
    }


    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn add_diversity_symbol(&mut self, sy_fqn : &String, sy_type : &TD_DataType) -> usize {
        let new_symbol_id = self.symbol_counter;
        self.symbol_counter = self.symbol_counter +1;
        self.symbol_diversity_names.insert( new_symbol_id, sy_fqn.clone());
        self.diversity_symbols_reverse_map.insert( sy_fqn.clone(), new_symbol_id );
        self.symbol_types.insert( new_symbol_id, sy_type.clone());
        return new_symbol_id;
    }

    pub fn get_sy_diversity_name(&self, sy_id: usize) -> Result<String, HibouCoreError> {
        match self.symbol_diversity_names.get(&sy_id) {
            None => {
                return Err( HibouCoreError::UnknownSymbol(Some(sy_id),None) );
            },
            Some(sy_diversity_name) => {
                return Ok(sy_diversity_name.clone());
            }
        }
    }

    pub fn get_sy_type(&self, sy_id : usize) -> Result<TD_DataType,HibouCoreError> {
        match self.symbol_types.get(&sy_id) {
            None => {
                return Err( HibouCoreError::UnknownSymbol(Some(sy_id),None) );
            },
            Some( sy_type ) => {
                return Ok( sy_type.clone() );
            }
        }
    }

    pub fn get_sy_type_from_fqn(&self, sy_fqn : &String) -> Result<TD_DataType,HibouCoreError> {
        match self.diversity_symbols_reverse_map.get(sy_fqn) {
            None => {
                return Err( HibouCoreError::UnknownSymbol(None,Some(sy_fqn.clone()) ) );
            },
            Some( sy_id ) => {
                return self.get_sy_type(*sy_id);
            }
        }
    }

    pub fn get_sy_id_from_fqn(&self, sy_fqn : &String) -> Result<usize,HibouCoreError> {
        match self.diversity_symbols_reverse_map.get(sy_fqn) {
            None => {
                return Err( HibouCoreError::UnknownSymbol(None,Some(sy_fqn.clone()) ) );
            },
            Some( sy_id ) => {
                return Ok(*sy_id);
            }
        }
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    fn create_variable_instance(&mut self, parent_vr_id : usize) -> usize {
        let new_vr_instance_num : u32;
        match self.vr_instances_count.get(&parent_vr_id) {
            None => {
                new_vr_instance_num = 1;
            },
            Some( instance_num ) => {
                new_vr_instance_num = *instance_num;
            }
        }
        self.vr_instances_count.insert(parent_vr_id, new_vr_instance_num+1 );
        // ***
        let new_vr_unique_id = self.vr_id_counter;
        self.vr_id_counter = self.vr_id_counter +1;
        self.vr_originals.insert(new_vr_unique_id, (parent_vr_id, new_vr_instance_num));
        return new_vr_unique_id;
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn open_scope(&mut self, gen_ctx : &GeneralContext, scope : &Vec<usize>, interaction : &Interaction) -> Interaction {
        let mut mapping : HashMap<usize,usize> = HashMap::new();
        for vr_id in scope {
            let new_vr_id = self.create_variable_instance(*vr_id);
            mapping.insert(*vr_id,new_vr_id);
            if self.is_clock(gen_ctx,*vr_id).unwrap() {
                self.active_clocks.insert(new_vr_id);
            }
        }
        return interaction.apply_variable_mapping(&mapping);
    }

}

