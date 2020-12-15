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

use std::collections::HashMap;
use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::td_type::TD_DataType;

use crate::tools::fold_vec_to_string;

use crate::xlia::xlia_build_name_tools::*;
use crate::xlia::data::{td_generic_to_xlia,td_bool_to_xlia};
use crate::xlia::model_initialization::make_lifeline_initialization_action;
use crate::process::hibou_process::HibouProcessTemporality;

pub fn generate_xlia_model(gen_ctx : &GeneralContext,
                           exe_ctx : &ExecutionContext,
                           interaction : &Interaction,
                           temporality : &HibouProcessTemporality) -> String {
    let mut xlia_model : String = r#"@xlia< system , 1.0 >:"#.to_string();
    xlia_model.push_str("\n");
    match temporality {
        HibouProcessTemporality::Timed => {
            xlia_model.push_str("timed system <or> HIBOU {\n");
        },
        HibouProcessTemporality::UnTimed => {
            xlia_model.push_str("system <or> HIBOU {\n");
        }
    }

    //xlia_model.push_str("@public:\n");


    let mut variable_declaration : String = "\t@property:\n".to_string();

    // ***
    match temporality {
        HibouProcessTemporality::Timed => {
            variable_declaration.push_str("\tvar clock last_lf_compare_clock;\n");
            variable_declaration.push_str("\tvar float trace_delay;\n");
        },
        HibouProcessTemporality::UnTimed => {
            // nothing
        }
    }

    // ***
    let mut trace_compare_actions : Vec<String> = Vec::new();
    let mut ms_id : usize = 0;
    for (ms_name,ms_args) in gen_ctx.get_ms_specs() {
        //let mut signal_string : String = format!("\tsignal {}",ms_name);
        let mut compare_action_string = format!("\tmachine action_compare_ms_{} {{\n", ms_name);
        compare_action_string.push_str("\t@moe:\n");
        compare_action_string.push_str("\t\t@run{\n");
        match temporality {
            HibouProcessTemporality::Timed => {
                compare_action_string.push_str("\t\t\t// time does not flow in this action\n");
                compare_action_string.push_str( "\t\t\tguard($delay == 0.0);\n" );
                compare_action_string.push_str("\t\t\t// to compare timed trace delay\n");
                compare_action_string.push_str( "\t\t\tguard( last_lf_compare_clock == trace_delay );\n" );
            },
            HibouProcessTemporality::UnTimed => {
                // nothing
            }
        }
        compare_action_string.push_str("\t\t\t// values of ms_M_pr_P kept from last symbolic step\n");
        compare_action_string.push_str("\t\t\t// values of trace_ms_M_pr_P provided by HIBOU\n");
        let mut arg_count = 0;
        if ms_args.len() > 0 {
            for ( arg_type, _) in ms_args {
                // ***
                let xlia_arg_type = arg_type.as_xlia_str();
                let parameter_diversity_name = message_parameter_diversity_name(gen_ctx,ms_id,arg_count);
                variable_declaration.push_str(&format!("\tvar {} {};\n",xlia_arg_type,parameter_diversity_name));
                let trace_parameter_diversity_name = trace_message_parameter_diversity_name(gen_ctx,ms_id,arg_count);
                variable_declaration.push_str(&format!("\tvar {} {};\n",xlia_arg_type,trace_parameter_diversity_name));
                // ***
                compare_action_string.push_str( &format!("\t\t\tguard({} == {});\n", parameter_diversity_name, trace_parameter_diversity_name)  );
                // ***
                /*signal_string.push_str(&arg_type.as_xlia_str());
                if arg_count < ms_args.len() {
                    signal_string.push_str(",");
                }*/
                arg_count = arg_count + 1;
            }
            //signal_string.push_str(")");
        }
        // ***
        match temporality {
            HibouProcessTemporality::Timed => {
                compare_action_string.push_str("\t\t\t// we now reset the last_lf_compare_clock because we are in the moment of the latest visible action on that lifeline \n");
                compare_action_string.push_str("\t\t\tlast_lf_compare_clock := 0.0;\n");
            },
            HibouProcessTemporality::UnTimed => {
                // nothing
            }
        }
        // ***
        compare_action_string.push_str("\t\t}\n");
        compare_action_string.push_str("\t}\n");
        trace_compare_actions.push(compare_action_string);
        //signal_string.push_str(";\n");
        //xlia_model.push_str(&signal_string);
        ms_id = ms_id +1;
    }

    let mut open_scope_action_string  = "\tmachine <start> action_open_scopes {\n".to_string();
    open_scope_action_string.push_str("\t@moe:\n");
    open_scope_action_string.push_str("\t\t@run{\n");
    open_scope_action_string.push_str("\t\t\t// creates a new place in each meta-variable vector allowing designation of scoped variables\n");
    open_scope_action_string.push_str("\t\t\t// called once at the beginning so that every variable vector in the DIVERSITY model has exactly one place for the original instance of the HIBOU meta-variable\n");
    open_scope_action_string.push_str("\t\t\t// called later every time a scope operator is opened in the execution\n");
    let vr_names = gen_ctx.get_vr_names();
    let vr_types = gen_ctx.get_vr_types();
    for vr_id in 0..vr_names.len() {
        let vr_index_name = variable_array_index_diversity_name(gen_ctx,vr_id);
        let vr_vector_name = variable_vector_diversity_name(gen_ctx,vr_id);
        let vr_base_for_newfresh_name = variable_base_for_newfresh_diversity_name(gen_ctx,vr_id);

        // ***
        let vr_type_xlia_str : String;
        if gen_ctx.is_clock( vr_id ) {
            vr_type_xlia_str = "clock".to_string();
        } else {
            let vr_type : &TD_DataType = vr_types.get(vr_id).unwrap();
            vr_type_xlia_str = vr_type.as_xlia_str();
        }
        // ***
        let vr_base_for_newfresh_xlia_dec = format!("\tvar {} {};\n",vr_type_xlia_str,vr_base_for_newfresh_name);
        variable_declaration.push_str(&vr_base_for_newfresh_xlia_dec);
        // ***
        let var_vector_xlia_dec = format!("\tvar vector<{}> {};\n",vr_type_xlia_str,vr_vector_name);
        variable_declaration.push_str(&var_vector_xlia_dec);
        // ***
        let var_index_dec  = format!("\tvar int {};\n", vr_index_name);
        variable_declaration.push_str(&var_index_dec);
        // ***
        open_scope_action_string.push_str( &format!("\t\t\t{} <=< newfresh({});\n", vr_vector_name, vr_base_for_newfresh_name) );
    }
    open_scope_action_string.push_str("\t\t}\n");
    open_scope_action_string.push_str("\t}\n");

    xlia_model.push_str("@composite:");
    xlia_model.push_str("\n");

    let mut lifelines_actions : HashMap<usize,Vec<String>> = HashMap::new();
    generate_xlia_lifelines(gen_ctx, exe_ctx, interaction, &mut lifelines_actions, Vec::new());

    for lf_id in 0..gen_ctx.get_lf_num() {
        let lf_name = gen_ctx.get_lf_name(lf_id).unwrap();
        let mut lifeline_string : String = format!("\tlifeline machine <or> {} {{\n",lf_name);
        // ***
        lifeline_string.push_str("\t@public:\n");
        lifeline_string.push_str("\t\tport output hevent(string);\n");
        // ***
        lifeline_string.push_str(&variable_declaration);
        lifeline_string.push_str("\t@composite:\n");
        // ***
        lifeline_string.push_str( &open_scope_action_string );
        lifeline_string.push_str( &make_lifeline_initialization_action(gen_ctx,exe_ctx,lf_id) );
        // ***
        match lifelines_actions.get(&lf_id) {
            None => {},
            Some(lf_actions) => {
                for lf_act in lf_actions {
                    lifeline_string.push_str(lf_act);
                }
            }
        }
        // ***
        for lf_trace_compare_act in &trace_compare_actions {
            lifeline_string.push_str(lf_trace_compare_act);
        }
        // ***
        lifeline_string.push_str("\t}\n");
        // ***
        xlia_model.push_str(&lifeline_string);
    }

    xlia_model.push_str("@com:\n");
    xlia_model.push_str("\tconnect<env>{\n");
    for lf_id in 0..gen_ctx.get_lf_num() {
        let lf_name = gen_ctx.get_lf_name(lf_id).unwrap();
        xlia_model.push_str(&format!("\t\toutput {}->hevent;\n",lf_name) );
    }
    xlia_model.push_str("\t}\n");
    xlia_model.push_str("}");

    return xlia_model;
}


fn update_xlia_lifelines_from_lf_act(gen_ctx : &GeneralContext,
                                         exe_ctx : &ExecutionContext,
                                         lf_id : usize,
                                         preamble : &Vec<ActionAmbleItem>,
                                         postamble : &Vec<ActionAmbleItem>,
                                         ms_id : usize,
                                         params : &Vec<ValueOrNewFresh>,
                                         is_emission : bool,
                                         is_target : bool,
                                         lifelines_actions : &mut HashMap<usize,Vec<String>>,
                                         relative_position : Vec<u32>) {

    // ***
    let action_name = action_diversity_name(&relative_position);
    //println!("creating on lifeline {} action on position {}",lf_name,action_name);
    // ***
    let mut xlia_action_str : String = format!("\tmachine {} {{\n",action_name);
    // ***
    xlia_action_str.push_str("\t@moe:\n");
    xlia_action_str.push_str("\t\t@run{\n");
    xlia_action_str.push_str("\t\t// values of index_V provided by HIBOU\n");
    // ***
    xlia_action_str.push_str("\t\t\t// Pre-Amble\n");
    for amble_item in preamble {
        match amble_item {
            ActionAmbleItem::Assignment( vr_id, value_or_new_fresh ) => {
                match value_or_new_fresh {
                    ValueOrNewFresh::NewFresh => {
                        let vr_base_for_newfresh_name = variable_base_for_newfresh_diversity_name(gen_ctx,*vr_id);
                        let vr_complete_name_within_array = variable_diversity_name(gen_ctx,*vr_id);
                        xlia_action_str.push_str( &format!("\t\t\t{} = newfresh({});\n", vr_complete_name_within_array, vr_base_for_newfresh_name) );
                    },
                    ValueOrNewFresh::Value( td_gen ) => {
                        xlia_action_str.push_str( &format!("\t\t\t{} = {};\n",variable_diversity_name(gen_ctx,*vr_id),td_generic_to_xlia(gen_ctx,td_gen))   );
                    }
                }
            },
            ActionAmbleItem::Guard( td_bool ) => {
                xlia_action_str.push_str( &format!("\t\t\tguard({});\n",td_bool_to_xlia(gen_ctx,td_bool))   );
            },
            ActionAmbleItem::Reset( vr_id ) => {
                let vr_complete_name_within_array = variable_diversity_name(gen_ctx,*vr_id);
                //xlia_action_str.push_str( &format!("\t\t\treset({});\n", vr_complete_name_within_array)   );
                xlia_action_str.push_str( &format!("\t\t\t{} := 0.0;\n", vr_complete_name_within_array)   );
            },
            _ => {
                panic!();
            }
        }
    }
    // ***
    if is_emission {
        xlia_action_str.push_str("\t\t\t// Emission - values of ms_M_pr_P computed by DIVERSITY - later queried by HIBOU\n");
        //let mut params_diversity_names : Vec<String> =
        for idx in 0..params.len() {
            let parameter_diversity_name = message_parameter_diversity_name(gen_ctx,ms_id,idx);
            let param_val = params.get(idx).unwrap();
            match param_val {
                ValueOrNewFresh::NewFresh => {
                    xlia_action_str.push_str( &format!("\t\t\tnewfresh({});\n", parameter_diversity_name)   );
                },
                ValueOrNewFresh::Value( td_gen ) => {
                    xlia_action_str.push_str( &format!("\t\t\t{} = {};\n",parameter_diversity_name,td_generic_to_xlia(gen_ctx,td_gen))   );
                }
            }
        }
        let hevent_str = format!("\"!{}\"", gen_ctx.get_ms_name(ms_id).unwrap());
        xlia_action_str.push_str( &format!("\t\t\toutput hevent ({});\n", hevent_str )   );
    } else {
        xlia_action_str.push_str("\t\t\t// Reception - values of ms_M_pr_P provided by HIBOU -\n");
        for idx in 0..params.len() {
            let parameter_diversity_name = message_parameter_diversity_name(gen_ctx,ms_id,idx);
            let param_val = params.get(idx).unwrap();
            match param_val {
                ValueOrNewFresh::NewFresh => {
                    if is_target {
                        // do nothing
                    } else {
                        xlia_action_str.push_str("\t\t\t// newfresh from the environment in a reception that isn't a target of an emission modelled in the DS\n");
                        xlia_action_str.push_str( &format!("\t\t\tnewfresh({});\n", parameter_diversity_name)   );
                    }
                },
                ValueOrNewFresh::Value( td_gen ) => {
                    // do nothing
                }
            }
        }
        let hevent_str = format!("\"?{}\"", gen_ctx.get_ms_name(ms_id).unwrap());
        xlia_action_str.push_str( &format!("\t\t\toutput hevent ({});\n", hevent_str )   );
    }
    // ***
    xlia_action_str.push_str("\t\t\t// Post-Amble\n");
    for amble_item in postamble {
        match amble_item {
            ActionAmbleItem::Assignment( vr_id, value_or_new_fresh ) => {
                match value_or_new_fresh {
                    ValueOrNewFresh::NewFresh => {
                        let vr_base_for_newfresh_name = variable_base_for_newfresh_diversity_name(gen_ctx,*vr_id);
                        let vr_complete_name_within_array = variable_diversity_name(gen_ctx,*vr_id);
                        xlia_action_str.push_str( &format!("\t\t\t{} = newfresh({});\n", vr_complete_name_within_array, vr_base_for_newfresh_name) );
                    },
                    ValueOrNewFresh::Value( td_gen ) => {
                        xlia_action_str.push_str( &format!("\t\t\t{} = {};\n",variable_diversity_name(gen_ctx,*vr_id),td_generic_to_xlia(gen_ctx,td_gen))   );
                    }
                }
            },
            ActionAmbleItem::Guard( td_bool ) => {
                xlia_action_str.push_str( &format!("\t\t\tguard({});\n",td_bool_to_xlia(gen_ctx,td_bool))   );
            },
            ActionAmbleItem::Reset( vr_id ) => {
                let vr_complete_name_within_array = variable_diversity_name(gen_ctx,*vr_id);
                xlia_action_str.push_str( &format!("\t\t\treset({});\n", vr_complete_name_within_array)   );
            },
            _ => {
                panic!();
            }
        }
    }
    // ***
    xlia_action_str.push_str("\t\t}\n");
    xlia_action_str.push_str("\t}\n");
    // ***
    match lifelines_actions.get(&lf_id) {
        None => {
            lifelines_actions.insert(lf_id,vec![xlia_action_str] );
        },
        Some( action_strings_vec ) => {
            let mut act_str_vec = action_strings_vec.clone();
            act_str_vec.push( xlia_action_str );
            lifelines_actions.insert(lf_id, act_str_vec );
        }
    }
}

fn generate_xlia_lifelines(gen_ctx : &GeneralContext,
                               exe_ctx : &ExecutionContext,
                               interaction : &Interaction,
                               lifelines_actions : &mut HashMap<usize,Vec<String>>,
                               relative_position : Vec<u32>) {
    match interaction {
        Interaction::Empty => {},
        Interaction::Action( obs_act ) => {
            match &obs_act.act_kind {
                ObservableActionKind::Emission( targets ) => {
                    update_xlia_lifelines_from_lf_act(gen_ctx,exe_ctx,obs_act.lf_act.lf_id,
                                                      &obs_act.lf_act.preamble,&obs_act.lf_act.postamble,obs_act.ms_id,&obs_act.params,true,false,
                                                      lifelines_actions,relative_position.clone());
                    // ***
                    let mut target_counter : u32 = 1;
                    for target in targets {
                        let mut rel_pos = relative_position.clone();
                        rel_pos.push(target_counter );
                        target_counter = target_counter +1;
                        update_xlia_lifelines_from_lf_act(gen_ctx,exe_ctx,target.lf_id,
                                                          &target.preamble,&target.postamble,obs_act.ms_id,&obs_act.params,false,true,
                                                          lifelines_actions,rel_pos);
                    }
                },
                ObservableActionKind::Reception => {
                    update_xlia_lifelines_from_lf_act(gen_ctx,exe_ctx,obs_act.lf_act.lf_id,
                                                      &obs_act.lf_act.preamble,&obs_act.lf_act.postamble,obs_act.ms_id,&obs_act.params,false,false,
                                                      lifelines_actions,relative_position.clone());
                }
            }
        },
        Interaction::Scope(_,sub_interaction) => {
            let mut rel_pos = relative_position.clone();
            rel_pos.push(1 );
            generate_xlia_lifelines(gen_ctx,exe_ctx,sub_interaction,lifelines_actions,rel_pos);
        },
        Interaction::Loop(_,sub_interaction) => {
            let mut rel_pos = relative_position.clone();
            rel_pos.push(1 );
            generate_xlia_lifelines(gen_ctx,exe_ctx,sub_interaction,lifelines_actions,rel_pos);
        },
        Interaction::Strict(subint1,subint2) => {
            {
                let mut rel_pos = relative_position.clone();
                rel_pos.push(1 );
                generate_xlia_lifelines(gen_ctx,exe_ctx,subint1,lifelines_actions,rel_pos);
            }
            {
                let mut rel_pos = relative_position.clone();
                rel_pos.push(2 );
                generate_xlia_lifelines(gen_ctx,exe_ctx,subint2,lifelines_actions,rel_pos);
            }
        },
        Interaction::Seq(subint1,subint2) => {
            {
                let mut rel_pos = relative_position.clone();
                rel_pos.push(1 );
                generate_xlia_lifelines(gen_ctx,exe_ctx,subint1,lifelines_actions,rel_pos);
            }
            {
                let mut rel_pos = relative_position.clone();
                rel_pos.push(2 );
                generate_xlia_lifelines(gen_ctx,exe_ctx,subint2,lifelines_actions,rel_pos);
            }
        },
        Interaction::Alt(subint1,subint2) => {
            {
                let mut rel_pos = relative_position.clone();
                rel_pos.push(1 );
                generate_xlia_lifelines(gen_ctx,exe_ctx,subint1,lifelines_actions,rel_pos);
            }
            {
                let mut rel_pos = relative_position.clone();
                rel_pos.push(2 );
                generate_xlia_lifelines(gen_ctx,exe_ctx,subint2,lifelines_actions,rel_pos);
            }
        },
        Interaction::Par(subint1,subint2) => {
            {
                let mut rel_pos = relative_position.clone();
                rel_pos.push(1 );
                generate_xlia_lifelines(gen_ctx,exe_ctx,subint1,lifelines_actions,rel_pos);
            }
            {
                let mut rel_pos = relative_position.clone();
                rel_pos.push(2 );
                generate_xlia_lifelines(gen_ctx,exe_ctx,subint2,lifelines_actions,rel_pos);
            }
        }
    }
}
