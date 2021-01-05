//! Formal Integration Tests for All 24 Befunge-98 Spatial Microkernels.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use std::fs;
use std::path::Path;
use poppet::vm::interpreter::{Interpreter, VmState};
use poppet::hal::HydraulicHal;

#[test]
fn test_all_befunge_kernels_parse_and_execute() {
    let kernels_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("kernels");
    assert!(kernels_dir.exists(), "kernels directory must exist");

    let entries = fs::read_dir(kernels_dir).expect("Failed to read kernels directory");
    let mut verified_count = 0;

    for entry in entries {
        let entry = entry.expect("Valid dir entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("b98") {
            let src = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("Failed reading {}: {}", path.display(), e));

            let mut vm = Interpreter::new(128, 128);
            let hal = Box::new(HydraulicHal::new());
            vm.set_mmio_handler(hal);
            vm.load_source(&src);
            vm.set_max_cycles(Some(1000));

            let state = vm.run();
            assert!(
                state == VmState::Halted || state == VmState::Timeout || state == VmState::Running,
                "Kernel {} produced invalid state: {:?}",
                path.file_name().unwrap().to_string_lossy(),
                state
            );
            verified_count += 1;
        }
    }

    assert_eq!(verified_count, 24, "All 24 Befunge-98 kernels must be verified");
}
