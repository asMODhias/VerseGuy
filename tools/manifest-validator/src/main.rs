use anyhow::{Context, Result};
use glob::glob;
use serde::Deserialize;
use std::env;
use std::fs;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PluginSection {
    id: Option<String>,
    name: Option<String>,
    version: Option<String>,
    author: Option<String>,
    description: Option<String>,
    license_required: Option<String>,
    core_version_min: Option<String>,
    sdk_version: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Capabilities {
    required: Option<Vec<String>>,
    optional: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    plugin: Option<PluginSection>,
    capabilities: Option<Capabilities>,
}

fn validate_manifest(path: &str) -> Result<Vec<String>> {
    let s = fs::read_to_string(path).with_context(|| format!("reading {}", path))?;
    let manifest: Manifest =
        toml::from_str(&s).with_context(|| format!("parsing TOML {}", path))?;
    let mut errs = Vec::new();

    if let Some(plugin) = manifest.plugin {
        if plugin.id.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            errs.push("missing or empty plugin.id".into());
        }
        if plugin.name.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            errs.push("missing or empty plugin.name".into());
        }
        if plugin
            .version
            .as_ref()
            .map(|s| s.is_empty())
            .unwrap_or(true)
        {
            errs.push("missing or empty plugin.version".into());
        }
        if plugin.core_version_min.is_none() {
            errs.push("missing plugin.core_version_min".into());
        }
        if plugin.sdk_version.is_none() {
            errs.push("missing plugin.sdk_version".into());
        }
    } else {
        errs.push("missing [plugin] section".into());
    }

    if let Some(cap) = manifest.capabilities {
        if cap.required.as_ref().map(|v| v.is_empty()).unwrap_or(true) {
            errs.push("capabilities.required missing or empty".into());
        }
    } else {
        errs.push("missing [capabilities] section".into());
    }

    Ok(errs)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let patterns = if args.is_empty() {
        vec!["plugins/**/manifest.toml".to_string()]
    } else {
        args
    };

    let mut any_errors = false;
    for pat in patterns {
        for entry in glob(&pat).context("invalid glob pattern")? {
            match entry {
                Ok(path) => {
                    let p = path.to_string_lossy().to_string();
                    print!("Validating {}... ", p);
                    match validate_manifest(&p) {
                        Ok(errs) => {
                            if errs.is_empty() {
                                println!("OK");
                            } else {
                                any_errors = true;
                                println!("FAILED");
                                for e in errs {
                                    println!("  - {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            any_errors = true;
                            println!("ERROR parsing manifest: {}", e);
                        }
                    }
                }
                Err(e) => println!("Glob error: {}", e),
            }
        }
    }

    if any_errors {
        anyhow::bail!("validation failed")
    }
    Ok(())
}
