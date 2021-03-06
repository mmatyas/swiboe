// Copyright (c) The Swiboe development team. All rights reserved.
// Licensed under the Apache License, Version 2.0. See LICENSE.txt
// in the project root for license information.

#![feature(test)]

extern crate serde;
extern crate serde_json;
extern crate swiboe;
extern crate tempdir;
extern crate test;

#[path="../tests/support/mod.rs"] mod support;

use support::{TestHarness};
use swiboe::client::Client;
use swiboe::plugin_buffer;
use swiboe::rpc;
use test::Bencher;


// On my macbook: 293,350 ns/iter (+/- 28,545)
#[bench]
fn bench_create_and_delete_buffers(b: &mut Bencher) {
    let t = TestHarness::new();
    let active_client = Client::connect_unix(&t.socket_name).unwrap();

    b.iter(|| {
        let new_response: plugin_buffer::NewResponse = match active_client.call(
            "buffer.new", &plugin_buffer::NewRequest {
                content: Some("bli\nbla\nblub".into()),
            }).wait().unwrap()
        {
            rpc::Result::Ok(value) => serde_json::from_value(value).unwrap(),
            err => panic!("{:?}", err),
        };

        let _: plugin_buffer::DeleteResponse = match active_client.call(
            "buffer.delete", &plugin_buffer::DeleteRequest {
                buffer_index: new_response.buffer_index
            }).wait().unwrap() {
            rpc::Result::Ok(value) => serde_json::from_value(value).unwrap(),
            err => panic!("{:?}", err),
        };
    });
}
