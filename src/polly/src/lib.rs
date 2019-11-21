// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
extern crate epoll;
extern crate libc;
extern crate utils;
#[macro_use]
extern crate bitflags;

pub mod event_manager;
pub mod pollable;
