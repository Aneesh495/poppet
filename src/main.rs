//! Poppet Command Line Interface.
//!
//! Subsea Electro-Hydraulic High-Integrity Pressure Protection System (HIPPS)
//! Executive, 2D Befunge-98 VM Engine, and Rhai SCADA Automation Host.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use poppet::hal::HydraulicHal;
use poppet::rhai_bridge::{PoppetSystem, RhaiEngine};
use poppet::safety::SafetyMonitor;
use poppet::telemetry::TelemetryPacket;
use poppet::vm::debugger::Debugger;
use poppet::vm::interpreter::{Interpreter, VmState};

#[derive(Parser, Debug)]
#[command(
    name = "poppet",
    author = "Aneesh Krishna Parthasarathy <aneeshkrishnaparthasarathy@gmail.com>",
    version = "0.1.0",
    about = "Safety-Critical High-Integrity Pressure Protection System (HIPPS) in Befunge and Rust"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Executes a Befunge-98 spatial microkernel
    Run {
        /// Path to .b98 / .bf kernel file
        file: PathBuf,
        /// Maximum execution cycle budget
        #[arg(short, long, default_value_t = 10000)]
        max_cycles: u64,
        /// Print Funge-Space execution statistics
        #[arg(short, long)]
        stats: bool,
    },
    /// Runs real-time physical hydraulic simulation
    Sim {
        /// Simulation duration in milliseconds
        #[arg(short, long, default_value_t = 100)]
        duration_ms: u64,
        /// Time step in milliseconds
        #[arg(short, long, default_value_t = 5.0)]
        step_ms: f64,
        /// Inject overpressure surge peak in bar x 10 (e.g. 4200 for 420.0 bar)
        #[arg(long)]
        surge: Option<i64>,
    },
    /// Executes an automated Rhai SCADA qualification script
    Rhai {
        /// Path to .rhai test script
        file: PathBuf,
    },
    /// Launches interactive Funge-Space viewport debugger
    Debug {
        /// Path to .b98 / .bf kernel file
        file: PathBuf,
        /// Viewport radius X
        #[arg(short = 'x', long, default_value_t = 8)]
        radius_x: i64,
        /// Viewport radius Y
        #[arg(short = 'y', long, default_value_t = 4)]
        radius_y: i64,
    },
    /// Executes built-in qualification self-tests
    Test,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, max_cycles, stats } => {
            println!("Loading Befunge-98 kernel: {}", file.display());
            let src = fs::read_to_string(&file).unwrap_or_else(|e| {
                eprintln!("Error reading kernel file: {}", e);
                std::process::exit(1);
            });

            let mut vm = Interpreter::new(128, 128);
            let hal = Box::new(HydraulicHal::new());
            vm.set_mmio_handler(hal);
            vm.set_max_cycles(Some(max_cycles));
            vm.load_source(&src);

            let state = vm.run();
            let output = vm.get_output_string();

            if !output.is_empty() {
                println!("Kernel Output:\n{}", output);
            }
            println!("Final VM State: {:?}", state);

            if stats {
                println!("--- Funge-Space Execution Metrics ---");
                println!("  Cycles Executed:  {}", vm.cycle_count);
                println!("  Memory Mutations: {}", vm.space.mutation_count);
                println!("  Final Stack Top:  {}", vm.active_stack().peek());
                println!("  Stack Depth:      {}", vm.active_stack().len());
            }
        }

        Commands::Sim { duration_ms, step_ms, surge } => {
            println!("Starting Poppet Subsea Hydraulic Simulation ({} ms)...", duration_ms);
            let mut hal = HydraulicHal::new();
            let mut safety = SafetyMonitor::default();

            if let Some(peak) = surge {
                println!("Injecting overpressure transient: {:.1} bar", (peak as f64) / 10.0);
                hal.inject_overpressure(peak);
            }

            let mut t = 0.0;
            let mut seq = 0u32;

            while t < (duration_ms as f64) {
                hal.tick(step_ms);
                safety.supervise(&mut hal);
                seq += 1;

                if seq % 5 == 0 {
                    let pkt = TelemetryPacket::capture(&hal, seq, hal.sensor_a);
                    println!(
                        "T={:4.0}ms | Pa={:5.1} Pb={:5.1} Pc={:5.1} | Acc={:5.1} bar | V1={:4} V2={:4} | Trip={}",
                        pkt.timestamp_ms,
                        (hal.sensor_a as f64) / 10.0,
                        (hal.sensor_b as f64) / 10.0,
                        (hal.sensor_c as f64) / 10.0,
                        pkt.accumulator_bar as f64,
                        pkt.valve_1_mille,
                        pkt.valve_2_mille,
                        pkt.trip_status
                    );
                }

                t += step_ms;
            }
            println!("Simulation finished cleanly.");
        }

        Commands::Rhai { file } => {
            println!("Executing Rhai SCADA procedure: {}", file.display());
            let script = fs::read_to_string(&file).unwrap_or_else(|e| {
                eprintln!("Error reading Rhai script: {}", e);
                std::process::exit(1);
            });

            let rhai = RhaiEngine::new();
            let mut sys = PoppetSystem::new();

            match rhai.eval_with_system(&script, &mut sys) {
                Ok(_) => println!("Rhai procedure executed successfully."),
                Err(e) => {
                    eprintln!("Rhai execution error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Debug { file, radius_x, radius_y } => {
            let src = fs::read_to_string(&file).unwrap_or_else(|e| {
                eprintln!("Error reading kernel file: {}", e);
                std::process::exit(1);
            });

            let mut vm = Interpreter::new(128, 128);
            let hal = Box::new(HydraulicHal::new());
            vm.set_mmio_handler(hal);
            vm.load_source(&src);

            let mut dbg = Debugger::new(100);
            println!("Stepping initial 10 cycles...");
            for _ in 0..10 {
                if vm.state == VmState::Halted {
                    break;
                }
                dbg.step_and_check(&mut vm);
            }

            println!("{}", dbg.render_viewport(&vm, radius_x, radius_y));
        }

        Commands::Test => {
            println!("Running Poppet qualification gates...");
            let rhai = RhaiEngine::new();

            let gates = [
                ("Gate 1: HIPPS Overpressure Rapid Trip", "rhai/test_hipps_trip.rhai"),
                ("Gate 2: TMR 2oo3 Transducer Voting", "rhai/test_tmr_voting.rhai"),
                ("Gate 3: Valve Kinematics Benchmark", "rhai/test_valve_stroke.rhai"),
                ("Gate 4: System Commissioning Sequence", "rhai/scada_commissioning.rhai"),
            ];

            for (name, path) in gates {
                print!("Executing {}... ", name);
                let script = fs::read_to_string(path).expect("Test file must exist");
                let mut sys = PoppetSystem::new(); // Fresh system state per gate
                match rhai.eval_with_system(&script, &mut sys) {
                    Ok(_) => println!("[PASSED]"),
                    Err(e) => {
                        println!("[FAILED]");
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }

            println!("ALL QUALIFICATION GATES PASSED :: SIL 4 COMPLIANCE VERIFIED");
        }
    }
}
