
use std::process::Command;

use std::fs;

static VENV_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/_venv");
static VENV_PY: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/_venv/bin/python");
// static VENV_INST_2: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/_venv/bin/python2.7/site-packages/");
// static VENV_INST_3: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/_venv/bin/python3.6/site-packages/");
static INT_TEST: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/integration_test.py");
static SETUP_PY: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/setup.py");

// Create a python virtual environment
fn create_venv(pyname: &str, venv_mod: &str) {
    let status = Command::new(pyname)
        .args(&["-m", venv_mod, VENV_DIR])
        .status()
        .expect("failed to create virtual environment for integration test!");

    assert!(status.success());
}

fn install_pkg_to_venv() {
    let status = Command::new(VENV_PY)
        .args(&[SETUP_PY, "install"])
        .status()
        .expect("error installing package to virtual environment");

    assert!(status.success());
}

fn run_integration_test() {
    let status = Command::new(VENV_PY)
        .args(&[INT_TEST])
        .status()
        .expect("error raised in integration test");

    assert!(status.success());
}

#[test]
fn integration_test() {
    if cfg!(target_os = "windows") {
        panic!("Windows is not supported yet for integration tests!");
    }
    
    create_venv("python2", "virtualenv");
    install_pkg_to_venv();
    run_integration_test();
    
    fs::remove_dir_all(VENV_DIR).expect("failed to remove temporary virtual environment for python");

    create_venv("python3", "venv");
    install_pkg_to_venv();
    run_integration_test();

    fs::remove_dir_all(VENV_DIR).expect("failed to remove temporary virtual environment for python");
}
