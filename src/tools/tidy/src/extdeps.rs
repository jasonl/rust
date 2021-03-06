// ! Check for external package sources. Allow only vendorable packages.

use std::fs;
use std::path::Path;

/// List of whitelisted sources for packages
const WHITELISTED_SOURCES: &[&str] = &[
    "\"registry+https://github.com/rust-lang/crates.io-index\"",
];

/// check for external package sources
pub fn check(path: &Path, bad: &mut bool) {
    // Cargo.lock of rust (tidy runs inside src/)
    let path = path.join("../Cargo.lock");

    // open and read the whole file
    let cargo_lock = t!(fs::read_to_string(&path));

    // process each line
    for line in cargo_lock.lines() {

        // consider only source entries
        if ! line.starts_with("source = ") {
            continue;
        }

        // extract source value
        let source = line.splitn(2, '=').nth(1).unwrap().trim();

        // ensure source is whitelisted
        if !WHITELISTED_SOURCES.contains(&&*source) {
            println!("invalid source: {}", source);
            *bad = true;
        }
    }
}
