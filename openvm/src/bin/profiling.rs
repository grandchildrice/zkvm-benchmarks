// ANCHOR: dependencies
use std::sync::Arc;
use std::time::Instant;

use eyre::Result;
use openvm_build::GuestOptions;
use openvm_sdk::{
    config::{AppConfig, SdkVmConfig},
    prover::AppProver,
    Sdk, StdIn,
};
use openvm_stark_sdk::config::FriParameters;
use utils::{ecdsa_input, profile::profile_func};

// ANCHOR_END: dependencies

#[allow(unused_variables, unused_doc_comments)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = ecdsa_input();
    // ANCHOR: vm_config
    let vm_config = SdkVmConfig::builder()
        .system(Default::default())
        .rv32i(Default::default())
        .rv32m(Default::default())
        .io(Default::default())
        .build();
    // ANCHOR_END: vm_config

    // ANCHOR: build
    // 1. Build the VmConfig with the extensions needed.
    let sdk = Sdk;

    // 2a. Build the ELF with guest options and a target filter.
    let guest_opts = GuestOptions::default();
    let target_path = "ecdsa-guest";
    let elf = sdk
        .build(guest_opts, target_path, &Default::default())
        .unwrap();
    // ANCHOR_END: build

    // ANCHOR: transpilation
    // 3. Transpile the ELF into a VmExe
    let exe = sdk.transpile(elf, vm_config.transpiler()).unwrap();
    // ANCHOR_END: transpilation

    // ANCHOR: execution
    // 4. Format your input into StdIn
    let mut stdin = StdIn::default();
    stdin.write(&input);

    // 5. Run the program
    let output = sdk
        .execute(exe.clone(), vm_config.clone(), stdin.clone())
        .unwrap();
    println!("public values output: {:?}", output);
    // ANCHOR_END: execution

    // ANCHOR: proof_generation
    // 6. Set app configuration
    let app_log_blowup = 2;
    let app_fri_params = FriParameters::standard_with_100_bits_conjectured_security(app_log_blowup);
    let app_config = AppConfig::new(app_fri_params, vm_config);

    // 7. Commit the exe
    let app_committed_exe = sdk.commit_app_exe(app_fri_params, exe).unwrap();

    // 8. Generate an AppProvingKey
    let app_pk = Arc::new(sdk.app_keygen(app_config).unwrap());

    // 9a. Generate a proof
    // let proof = sdk.generate_app_proof(app_pk.clone(), app_committed_exe.clone(), stdin.clone()).unwrap();
    // 9b. Generate a proof with an AppProver with custom fields
    let start = Instant::now();
    let app_prover = AppProver::new(app_pk.app_vm_pk.clone(), app_committed_exe.clone())
        .with_program_name("test_program");
    profile_func(
        || {
            let _proof = app_prover.generate_app_proof(stdin.clone());
        },
        "../.outputs/profiling/profile_openvm.pb",
    )?;

    Ok(())
}
