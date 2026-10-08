/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use std::arch::x86_64::__cpuid;

const LEAF_FEATURES: u32 = 0x0000_0001;
const LEAF_EXTENDED_MAX: u32 = 0x8000_0000;
const LEAF_EXTENDED_FEATURES: u32 = 0x8000_0001;

enum Leaf {
    Features,
    ExtendedFeatures,
}

const X86_64_V2: [(&str, Leaf, u32); 7] = [
    ("sse3", Leaf::Features, 0),
    ("ssse3", Leaf::Features, 9),
    ("cmpxchg16b", Leaf::Features, 13),
    ("sse4.1", Leaf::Features, 19),
    ("sse4.2", Leaf::Features, 20),
    ("popcnt", Leaf::Features, 23),
    ("lahf_lm", Leaf::ExtendedFeatures, 0),
];

pub fn assert_x86_64_v2() {
    let features = __cpuid(LEAF_FEATURES).ecx;
    let extended_features = if __cpuid(LEAF_EXTENDED_MAX).eax >= LEAF_EXTENDED_FEATURES {
        __cpuid(LEAF_EXTENDED_FEATURES).ecx
    } else {
        0
    };

    let mut missing = String::new();
    for (name, leaf, bit) in X86_64_V2 {
        let register = match leaf {
            Leaf::Features => features,
            Leaf::ExtendedFeatures => extended_features,
        };
        if register & (1 << bit) == 0 {
            if !missing.is_empty() {
                missing.push_str(", ");
            }
            missing.push_str(name);
        }
    }

    if !missing.is_empty() {
        panic!(
            "This build of Stalwart requires an x86-64-v2 compatible CPU but the following instructions are not available: {missing}. If Stalwart runs in a virtual machine, set its CPU type to 'host' or 'x86-64-v2'."
        );
    }
}
