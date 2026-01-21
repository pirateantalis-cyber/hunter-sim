//! CLI entry point for Hunter Simulator

use clap::{Parser, ValueEnum};
use hunter_sim_lib::{
    config::BuildConfig,
    simulation::run_and_aggregate,
};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Parser, Debug)]
#[command(name = "hunter-sim")]
#[command(version = "1.0")]
#[command(about = "High-performance Hunter Simulator for CIFI idle game", long_about = None)]
struct Args {
    /// Path to the build configuration file (YAML or JSON)
    #[arg(short, long)]
    config: PathBuf,

    /// Number of simulations to run
    #[arg(short, long, default_value = "100")]
    num_sims: usize,

    /// Use parallel processing
    #[arg(short, long, default_value = "false")]
    parallel: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    output: OutputFormat,

    /// Show timing information
    #[arg(short, long, default_value = "false")]
    timing: bool,
}

fn main() {
    let args = Args::parse();

    // Load config
    let config = match BuildConfig::from_file(&args.config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    // Run simulations
    let start = Instant::now();
    let stats = run_and_aggregate(&config, args.num_sims, args.parallel);
    let elapsed = start.elapsed();

    // Output results
    match args.output {
        OutputFormat::Text => {
            println!("=== Hunter Simulation Results ===");
            println!("Simulations: {}", args.num_sims);
            println!();
            println!("Average Final Stage: {:.2} ± {:.2}", stats.avg_stage, stats.std_stage);
            println!("Stage Range: {} - {}", stats.min_stage, stats.max_stage);
            println!();
            println!("Average Elapsed Time: {:.2}s", stats.avg_time);
            println!("Average Total Loot: {:.0}", stats.avg_loot);
            println!();
            println!("--- Combat Stats ---");
            println!("Avg Damage Dealt: {:.0}", stats.avg_damage);
            println!("Avg Damage Taken: {:.0}", stats.avg_damage_taken);
            println!("Avg Damage Mitigated: {:.0}", stats.avg_mitigated);
            println!("Avg Lifesteal: {:.0}", stats.avg_lifesteal);
            println!();
            println!("Avg Attacks: {:.0}", stats.avg_attacks);
            println!("Avg Crits: {:.0}", stats.avg_crits);
            println!("Avg Kills: {:.0}", stats.avg_kills);
            println!("Avg Evades: {:.0}", stats.avg_evades);
            println!("Avg Effect Procs: {:.0}", stats.avg_effect_procs);
            println!("Avg Stun Duration: {:.2}s", stats.avg_stun_duration);
            
            if args.timing {
                println!();
                println!("--- Performance ---");
                println!("Total time: {:.3}s", elapsed.as_secs_f64());
                println!("Per simulation: {:.3}ms", elapsed.as_secs_f64() * 1000.0 / args.num_sims as f64);
                println!("Simulations/sec: {:.0}", args.num_sims as f64 / elapsed.as_secs_f64());
            }
        }
        OutputFormat::Json => {
            let output = serde_json::json!({
                "simulations": args.num_sims,
                "parallel": args.parallel,
                "elapsed_seconds": elapsed.as_secs_f64(),
                "stats": {
                    "avg_stage": stats.avg_stage,
                    "std_stage": stats.std_stage,
                    "min_stage": stats.min_stage,
                    "max_stage": stats.max_stage,
                    "avg_time": stats.avg_time,
                    "avg_loot": stats.avg_loot,
                    "avg_loot_per_hour": stats.avg_loot_per_hour,
                    "avg_damage": stats.avg_damage,
                    "avg_damage_taken": stats.avg_damage_taken,
                    "avg_mitigated": stats.avg_mitigated,
                    "avg_lifesteal": stats.avg_lifesteal,
                    "avg_attacks": stats.avg_attacks,
                    "avg_crits": stats.avg_crits,
                    "avg_kills": stats.avg_kills,
                    "avg_evades": stats.avg_evades,
                    "avg_effect_procs": stats.avg_effect_procs,
                    "avg_stun_duration": stats.avg_stun_duration,
                    "survival_rate": stats.survival_rate,
                    "boss1_survival": stats.boss1_survival,
                    "boss2_survival": stats.boss2_survival,
                    "boss3_survival": stats.boss3_survival,
                    "boss4_survival": stats.boss4_survival,
                    "boss5_survival": stats.boss5_survival,
                }
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
    }
}
