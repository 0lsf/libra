// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use libra_types::transaction::ScriptABI;
use move_core_types::language_storage::TypeTag;
use std::io::Read;

/// Support for code-generation in Python 3
pub mod python3;
/// Support for code-generation in Rust
pub mod rust;

/// Useful error message.
fn type_not_allowed(type_tag: &TypeTag) -> ! {
    panic!(
        "Transaction scripts cannot take arguments of type {}.",
        type_tag
    );
}

/// Read all ABI files in a directory.
pub fn read_abis<P: AsRef<std::path::Path>>(dir_path: P) -> anyhow::Result<Vec<ScriptABI>> {
    let mut abis = Vec::<ScriptABI>::new();
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let mut buffer = Vec::new();
        let mut f = std::fs::File::open(path)?;
        f.read_to_end(&mut buffer)?;
        abis.push(serde_json::from_slice(&buffer)?);
    }
    // Sorting scripts by alphabetical order.
    abis.sort_by(|a, b| a.name().cmp(b.name()));
    Ok(abis)
}