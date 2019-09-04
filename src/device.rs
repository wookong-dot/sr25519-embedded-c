// -*- mode: rust; -*-
//
// This file is part of sr25519-embedded-c.
// Copyright (c) 2017-2019 Chester Li and extropies.com
// See LICENSE for licensing information.
//
// Authors:
// - Chester Li<chester@lichester.com>

#[cfg(any(feature = "embedded"))]
use core::panic::PanicInfo;

/// when compile static lib
#[cfg(any(feature = "embedded"))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
	loop {}
}