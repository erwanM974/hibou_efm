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

use std::collections::HashSet;
use std::collections::btree_map::BTreeMap;

use crate::diversity::symbex_client::SymbexClient;
use crate::diversity::model_definition_request::ModelAlt;
use crate::diversity::*;



use crate::grpc_connect::xlia_reference_name_tools::*;
use crate::grpc_connect::calls::*;

use crate::core::syntax::data::builtin::integer::TD_Integer;

pub async fn symbex_post_process(client : &mut SymbexClient<tonic::transport::Channel>) {

    // ***
    let post_process_request = tonic::Request::new(PostProcessingRequest {
        enable_execution_graph: true,
    });
    println!("POST PROCESS REQUEST = {:?}", post_process_request);
    let pose_process_response = client.run_post_processor(post_process_request).await.unwrap();
    let post_process_reply = pose_process_response.into_inner();
    println!("POST PROCESS REPLY = {:?}", post_process_reply );
    // ***
}