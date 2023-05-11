use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn can_successfully_extract_btpog_demo_markers() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("ut99-strings")
        .unwrap()
        .args(["-l", "20", "./tests/assets/CTF-BT-andACTION-dbl.dem"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "BTPOG_SPAWN_MARKER:HO0VE7DUOPBJ5N2QWBX26FTZ",
        ))
        .stdout(predicate::str::contains(
            "BTPOG_1S_AFTER_SPAWN_MARKER:HO0VE7DUOPBJ5N2QWBX26FTZ",
        ))
        .stdout(predicate::str::contains(
            "BTPOG_CAP_MARKER:HO0VE7DUOPBJ5N2QWBX26FTZ",
        ));

    Ok(())
}
