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
use std::fs;
use std::fs::File;
use std::io::{Read,BufReader,BufRead,BufWriter,Write};

// ***
use std::process::{Command,Output};

// ***
use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::position::*;
use crate::core::syntax::interaction::Interaction;
use crate::core::trace::AnalysableMultiTrace;
use crate::core::syntax::action::*;


use crate::process::log::ProcessLogger;
use crate::rendering::textual::monochrome::position::position_to_text;
use crate::rendering::graphviz::graph::*;
use crate::rendering::graphviz::node_style::*;
use crate::rendering::graphviz::edge_style::*;
use crate::rendering::graphviz::common::*;
use crate::rendering::custom_draw::seqdiag::interaction::draw_interaction;
use crate::rendering::custom_draw::firing::draw_firing::{draw_firing_on_model_action,draw_firing_on_trace_against_model_action};
use crate::process::verdicts::CoverageVerdict;
use crate::core::trace::{TraceAction,TraceActionKind};

use crate::process::hibou_process::FilterEliminationKind;

use crate::rendering::process::verdict::*;
// ***

use crate::process::hibou_process::*;

pub enum GraphicProcessLoggerKind {
    svg,
    png
}

pub struct GraphicProcessLogger {
    log_name : String,
    file : File,
    kind:GraphicProcessLoggerKind
}

impl GraphicProcessLogger {
    pub fn new(log_name : String,kind:GraphicProcessLoggerKind) -> GraphicProcessLogger {
        let file = File::create(&format!("{:}.dot",log_name)).unwrap();
        // ***
        return GraphicProcessLogger{
            log_name,
            file,
            kind}
    }
}

impl ProcessLogger for GraphicProcessLogger {

    fn log_verdict(&mut self,
                   parent_state_id : u32,
                   verdict : &CoverageVerdict) {
        // ***
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        // ***
        let verdict_node_name = format!("v{:}", parent_state_id);
        // *****
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
        tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
        // *****
        node_gv_options.push( GraphvizNodeStyleItem::Label( verdict.to_string() ) );
        // *****
        let verdict_color = verdict.get_verdict_color();
        node_gv_options.push( GraphvizNodeStyleItem::Color( verdict_color.clone() ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontSize( 16 ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontName( "times-bold".to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Diamond) );
        node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );

        tran_gv_options.push( GraphvizEdgeStyleItem::Color( verdict_color ) );
        // *****
        let gv_edge = GraphVizEdge{origin_id : parent_interaction_node_name.clone(), target_id : verdict_node_name.clone(), style : tran_gv_options};
        let gv_node = GraphVizNode{id : verdict_node_name, style : node_gv_options};
        let mut string_to_write = gv_node.to_dot_string();
        string_to_write.push_str("\n");
        string_to_write.push_str(&gv_edge.to_dot_string());
        string_to_write.push_str("\n");
        // *****
        self.file.write( string_to_write.as_bytes() );
    }

    fn log_init(&mut self,
                interaction : &Interaction,
                gen_ctx : &GeneralContext,
                exe_ctx : &ExecutionContext,
                remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        // ***
        // empties temp directory if exists
        match fs::remove_dir_all("./temp") {
            Ok(_) => {
                // do nothing
            },
            Err(e) => {
                // do nothing
            }
        }
        // creates temp directory
        fs::create_dir_all("./temp").unwrap();
        // ***
        self.file.write("digraph G {\n".as_bytes() );
        // ***
        // ***
        let gv_node_path : String = format!("./temp/{:}_i1.png", self.log_name);
        draw_interaction(&gv_node_path, interaction, gen_ctx, exe_ctx, remaining_multi_trace);

        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        node_gv_options.push( GraphvizNodeStyleItem::Label("".to_owned()) );
        node_gv_options.push( GraphvizNodeStyleItem::Image( gv_node_path ) );
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );

        let gv_node = GraphVizNode{id : "i1".to_owned(), style : node_gv_options};
        let mut string_to_write = gv_node.to_dot_string();
        string_to_write.push_str("\n");
        self.file.write( string_to_write.as_bytes() );
    }

    fn log_term(&mut self,
                options_as_strs : &Vec<String>) {

        // *** LEGEND
        {
            let mut legend_str = String::new();
            for opt_str in options_as_strs {
                legend_str.push_str(opt_str);
                legend_str.push_str("\\l");
            }
            // ***
            let mut legend_node_gv_options : GraphvizNodeStyle = Vec::new();
            legend_node_gv_options.push( GraphvizNodeStyleItem::Label( legend_str ) );
            legend_node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
            legend_node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Bold,GvNodeStyleKind::Rounded]) );
            legend_node_gv_options.push( GraphvizNodeStyleItem::FontSize( 18 ) );
            // ***
            let legend_node = GraphVizNode{id : "legend".to_owned(), style : legend_node_gv_options};
            let legend_as_dot_str = format!("{}\n", legend_node.to_dot_string());
            self.file.write( legend_as_dot_str.as_bytes() );
        }
        // ***
        self.file.write( "}".as_bytes() );
        // ***
        match self.kind {
            GraphicProcessLoggerKind::png => {
                match Command::new("dot")
                    .arg("-Tpng")
                    .arg(&format!("{:}.dot",self.log_name))
                    .arg("-o")
                    .arg(&format!("{:}.png",self.log_name))
                    .output() {
                    Err(e) => {
                        println!("error while calling dot -Tpng : {:?}", e);
                    }
                    Ok( output ) => {
                        if !output.status.success() {
                            println!("could not generate png graph");
                            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                }
            },
            GraphicProcessLoggerKind::svg => {
                match Command::new("dot")
                    .arg("-Tsvg:cairo")
                    .arg(&format!("{:}.dot",self.log_name))
                    .arg("-o")
                    .arg(&format!("{:}.svg",self.log_name))
                    .output() {
                    Err(e) => {
                        println!("error while calling dot -Tsvg:cairo : {:?}", e);
                    }
                    Ok( output ) => {
                        if !output.status.success() {
                            println!("could not use cairo to generate svg graph with embedded interaction images...");
                            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                            println!("... generating svg with external references");
                            match Command::new("dot")
                                .arg("-Tsvg")
                                .arg(&format!("{:}.dot",self.log_name))
                                .arg("-o")
                                .arg(&format!("{:}.svg",self.log_name))
                                .output() {
                                Err(e) => {
                                    println!("error while calling dot -Tsvg : {:?}", e);
                                }
                                Ok( output ) => {
                                    if !output.status.success() {
                                        println!("could not generate svg graph with external references");
                                        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                                        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }


    fn log_execution(&mut self,
                     gen_ctx : &GeneralContext,
                     parent_state_id : u32,
                     new_state_id : u32,
                     action_position : &Position,
                     trace_action : Option<&TraceAction>,
                     model_action : &ObservableAction,
                     new_interaction : &Interaction,
                     new_exe_ctx : &ExecutionContext,
                     remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        // *** Parent Interaction Node
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        // *** Firing Node
        let current_node_name = format!("i{:}", new_state_id);
        let firing_node_name = format!("f{:}", new_state_id);
        {
            let firing_node_path : String = format!("./temp/{:}_{}.png",  self.log_name ,firing_node_name);
            match trace_action {
                None => {
                    draw_firing_on_model_action(&firing_node_path, action_position, model_action, gen_ctx, new_exe_ctx);
                },
                Some( got_trace_action ) => {
                    draw_firing_on_trace_against_model_action(&firing_node_path, action_position, got_trace_action, model_action, gen_ctx, new_exe_ctx);
                }
            }
            // ***
            let mut firing_gv_node_options : GraphvizNodeStyle = Vec::new();
            firing_gv_node_options.push( GraphvizNodeStyleItem::Image( firing_node_path ) );
            firing_gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            firing_gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
            let firing_gv_node = GraphVizNode{id : firing_node_name.clone(), style : firing_gv_node_options};
            self.file.write( firing_gv_node.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Transition To Firing
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            let gv_edge = GraphVizEdge{origin_id : parent_interaction_node_name, target_id : firing_node_name.clone(), style : tran_gv_options};
            self.file.write( gv_edge.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Resulting Interaction Node
        {
            let gv_path : String = format!("./temp/{:}_{}.png",  self.log_name ,current_node_name);
            draw_interaction(&gv_path, new_interaction, gen_ctx, new_exe_ctx, remaining_multi_trace);
            // ***
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Image( gv_path ) );
            node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
            let gv_node = GraphVizNode{id : current_node_name.clone(), style : node_gv_options};
            self.file.write( gv_node.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Transition To Interaction Node
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            let gv_edge = GraphVizEdge{origin_id : firing_node_name, target_id : current_node_name, style : tran_gv_options};
            self.file.write( gv_edge.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
    }

    fn log_filtered(&mut self,
                    gen_ctx : &GeneralContext,
                    exe_ctx:&ExecutionContext,
                    parent_state_id : u32,
                    new_state_id : u32,
                    action_position : &Position,
                    action : &ObservableAction,
                    elim_kind : &FilterEliminationKind) {
        // *** Parent Interaction Node
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        // *** Firing Node
        let elim_node_name = format!("e{:}", new_state_id);
        let firing_node_name = format!("f{:}", new_state_id);
        {
            let firing_node_path : String = format!("./temp/{:}_{}.png",  self.log_name ,firing_node_name);
            draw_firing_on_model_action(&firing_node_path, action_position, action, gen_ctx, exe_ctx);
            // ***
            let mut firing_gv_node_options : GraphvizNodeStyle = Vec::new();
            firing_gv_node_options.push( GraphvizNodeStyleItem::Image( firing_node_path ) );
            firing_gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            firing_gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
            let firing_gv_node = GraphVizNode{id : firing_node_name.clone(), style : firing_gv_node_options};
            self.file.write( firing_gv_node.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Transition To Firing
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            let gv_edge = GraphVizEdge{origin_id : parent_interaction_node_name, target_id : firing_node_name.clone(), style : tran_gv_options};
            self.file.write( gv_edge.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Filtered Node
        {
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            // *****
            node_gv_options.push( GraphvizNodeStyleItem::Label( elim_kind.to_string() ) );
            // *****
            node_gv_options.push( GraphvizNodeStyleItem::Color( GraphvizColor::burlywood4 ) );
            node_gv_options.push( GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ) );
            node_gv_options.push( GraphvizNodeStyleItem::FontSize( 16 ) );
            node_gv_options.push( GraphvizNodeStyleItem::FontName( "times-bold".to_string() ) );
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Pentagon) );
            node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );

            let gv_node = GraphVizNode{id : elim_node_name.clone(), style : node_gv_options};
            self.file.write( gv_node.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Transition To Filtered Node
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::Color( GraphvizColor::burlywood4 ) );
            let gv_edge = GraphVizEdge{origin_id : firing_node_name, target_id : elim_node_name, style : tran_gv_options};
            self.file.write( gv_edge.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
    }

    fn log_unsat(&mut self,
                    gen_ctx : &GeneralContext,
                    exe_ctx:&ExecutionContext,
                    parent_state_id : u32,
                    new_state_id : u32,
                    action_position : &Position,
                    trace_action : Option<&TraceAction>,
                    model_action : &ObservableAction) {
        // *** Parent Interaction Node
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        // *** Firing Node
        let unsat_node_name = format!("u{:}", new_state_id);
        let firing_node_name = format!("f{:}", new_state_id);
        {
            let firing_node_path : String = format!("./temp/{:}_{}.png",  self.log_name ,firing_node_name);
            match trace_action {
                None => {
                    draw_firing_on_model_action(&firing_node_path, action_position, model_action, gen_ctx, exe_ctx);
                },
                Some( got_trace_action ) => {
                    draw_firing_on_trace_against_model_action(&firing_node_path, action_position, got_trace_action, model_action, gen_ctx, exe_ctx);
                }
            }
            // ***
            let mut firing_gv_node_options : GraphvizNodeStyle = Vec::new();
            firing_gv_node_options.push( GraphvizNodeStyleItem::Image( firing_node_path ) );
            firing_gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            firing_gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
            let firing_gv_node = GraphVizNode{id : firing_node_name.clone(), style : firing_gv_node_options};
            self.file.write( firing_gv_node.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Transition To Firing
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            let gv_edge = GraphVizEdge{origin_id : parent_interaction_node_name, target_id : firing_node_name.clone(), style : tran_gv_options};
            self.file.write( gv_edge.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** UNSAT Node
        {
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            // *****
            node_gv_options.push( GraphvizNodeStyleItem::Label( "UNSAT".to_string() ) );
            // *****
            node_gv_options.push( GraphvizNodeStyleItem::Color( GraphvizColor::red4 ) );
            node_gv_options.push( GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ) );
            node_gv_options.push( GraphvizNodeStyleItem::FontSize( 16 ) );
            node_gv_options.push( GraphvizNodeStyleItem::FontName( "times-bold".to_string() ) );
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Septagon) );
            node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );

            let gv_node = GraphVizNode{id : unsat_node_name.clone(), style : node_gv_options};
            self.file.write( gv_node.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Transition To UNSAT Node
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::Color( GraphvizColor::red4 ) );
            let gv_edge = GraphVizEdge{origin_id : firing_node_name, target_id : unsat_node_name, style : tran_gv_options};
            self.file.write( gv_edge.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
    }
}