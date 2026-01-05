// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) Andrea Righi <arighi@nvidia.com>
// Copyright (c) 2026 Rezky Nightky
// Modified by Rezky Nightky for scx_wolf
//
// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

fn main() {
    scx_cargo::BpfBuilder::new()
        .unwrap()
        .enable_intf("src/bpf/intf.h", "bpf_intf.rs")
        .enable_skel("src/bpf/main.bpf.c", "bpf")
        .build()
        .unwrap();
}
