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

use crate::process::hibou_process::NextToProcess;


pub struct ProcessQueue {
    queue : Vec<NextToProcess>
}

impl ProcessQueue {
    pub fn new() -> ProcessQueue {
        return ProcessQueue{queue:Vec::new()}
    }

    pub fn insert_item_left(&mut self,node:NextToProcess) {
        self.queue.insert(0,node);
    }

    pub fn insert_item_right(&mut self,node:NextToProcess) {
        self.queue.push(node);
    }

    pub fn get_next(&mut self) -> Option<NextToProcess> {
        if self.queue.len() > 0 {
            return Some( self.queue.remove(0) );
        } else {
            return None;
        }
    }

}