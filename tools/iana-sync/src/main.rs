use std::{collections::BTreeMap, env, fmt, fs, path::Path};

struct Registry {
    name: &'static str,
    url: &'static str,
    source: &'static str,
}

const REGISTRIES: &[Registry] = &[
    Registry {
        name: "SUIT Envelope Elements",
        url: "https://www.iana.org/assignments/suit/suit-envelope-elements.csv",
        source: "iana-suit/src/envelope.rs",
    },
    Registry {
        name: "SUIT Manifest Elements",
        url: "https://www.iana.org/assignments/suit/suit-manifest-elements.csv",
        source: "iana-suit/src/manifest.rs",
    },
    Registry {
        name: "SUIT Common Elements",
        url: "https://www.iana.org/assignments/suit/suit-common-elements.csv",
        source: "iana-suit/src/common.rs",
    },
    Registry {
        name: "SUIT Commands",
        url: "https://www.iana.org/assignments/suit/suit-commands.csv",
        source: "iana-suit/src/commands.rs",
    },
    Registry {
        name: "SUIT Parameters",
        url: "https://www.iana.org/assignments/suit/suit-parameters.csv",
        source: "iana-suit/src/parameters.rs",
    },
    Registry {
        name: "SUIT Text Values",
        url: "https://www.iana.org/assignments/suit/suit-text-values.csv",
        source: "iana-suit/src/text.rs",
    },
    Registry {
        name: "SUIT Component Text Values",
        url: "https://www.iana.org/assignments/suit/suit-component-text-values.csv",
        source: "iana-suit/src/component_text.rs",
    },
    Registry {
        name: "SUIT Report Elements",
        url: "https://www.iana.org/assignments/suit/suit-report-elements.csv",
        source: "iana-suit/src/report.rs",
    },
    Registry {
        name: "SUIT Record Elements",
        url: "https://www.iana.org/assignments/suit/suit-record-elements.csv",
        source: "iana-suit/src/record.rs",
    },
    Registry {
        name: "SUIT Report Reasons",
        url: "https://www.iana.org/assignments/suit/suit-report-reasons.csv",
        source: "iana-suit/src/report_reasons.rs",
    },
    Registry {
        name: "SUIT Capability Report Elements",
        url: "https://www.iana.org/assignments/suit/suit-capability-report-elements.csv",
        source: "iana-suit/src/capability_report.rs",
    },
];

#[derive(Debug)]
struct SyncError(String);

impl fmt::Display for SyncError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

fn main() {
    let result = match env::args().nth(1).as_deref() {
        Some("check") => run(false),
        Some("update") => run(true),
        _ => {
            eprintln!("usage: cargo run -p iana-sync -- <check|update>");
            std::process::exit(2);
        }
    };

    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(update: bool) -> Result<(), SyncError> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .map_err(|error| SyncError(format!("cannot locate repository root: {error}")))?;
    let mut errors = Vec::new();

    for registry in REGISTRIES {
        print!("{}: ", registry.name);
        let result = if update {
            update_registry(&root, registry)
        } else {
            check_registry(&root, registry)
        };

        match result {
            Ok(message) => println!("{message}"),
            Err(error) => {
                println!("FAILED");
                errors.push(error.0);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(SyncError(format!(
            "{} registry check(s) failed:\n{}",
            errors.len(),
            errors.join("\n")
        )))
    }
}

fn check_registry(root: &Path, registry: &Registry) -> Result<String, SyncError> {
    let expected = expected_labels(&download(registry)?, registry)?;
    let source = read_source(root, registry)?;
    let actual = source_labels(&source)?;
    let differences = differences(&expected, &actual);

    if differences.is_empty() {
        Ok(format!("ok ({} assigned labels)", expected.len()))
    } else {
        Err(SyncError(format!(
            "{}:\n{}",
            registry.name,
            differences.join("\n")
        )))
    }
}

fn update_registry(root: &Path, registry: &Registry) -> Result<String, SyncError> {
    let expected = expected_labels(&download(registry)?, registry)?;
    let path = root.join(registry.source);
    let source = fs::read_to_string(&path).map_err(|error| {
        SyncError(format!(
            "{}: cannot read {}: {error}",
            registry.name,
            path.display()
        ))
    })?;
    let actual = source_labels(&source)?;
    let missing: Vec<(i64, String)> = expected
        .iter()
        .filter(|(label, _)| !actual.contains_key(label))
        .map(|(label, name)| (*label, name.clone()))
        .collect();

    if missing.is_empty() {
        return Ok(format!("unchanged ({} assigned labels)", expected.len()));
    }

    let updated = append_constants(&source, &actual, &missing, registry)?;
    fs::write(&path, updated).map_err(|error| {
        SyncError(format!(
            "{}: cannot write {}: {error}",
            registry.name,
            path.display()
        ))
    })?;

    Ok(format!("added {} label(s)", missing.len()))
}

fn download(registry: &Registry) -> Result<String, SyncError> {
    let response = ureq::get(registry.url)
        .call()
        .map_err(|error| SyncError(format!("{}: IANA request failed: {error}", registry.name)))?;

    response.into_string().map_err(|error| {
        SyncError(format!(
            "{}: IANA response is not UTF-8: {error}",
            registry.name
        ))
    })
}

fn expected_labels(csv: &str, registry: &Registry) -> Result<BTreeMap<i64, String>, SyncError> {
    let rows = parse_csv(csv).map_err(|error| SyncError(format!("{}: {error}", registry.name)))?;
    let mut labels = BTreeMap::new();

    for row in rows.into_iter().skip(1) {
        let Some(raw_label) = row.first() else {
            continue;
        };
        let Some(label) = raw_label.trim().parse::<i64>().ok() else {
            continue;
        };
        let name = row.get(1).map(String::as_str).unwrap_or("Unnamed").trim();
        if name.is_empty() || name.eq_ignore_ascii_case("unassigned") {
            continue;
        }
        if labels.insert(label, name.to_owned()).is_some() {
            return Err(SyncError(format!(
                "{}: duplicate assigned label {label}",
                registry.name
            )));
        }
    }

    Ok(labels)
}

fn read_source(root: &Path, registry: &Registry) -> Result<String, SyncError> {
    let path = root.join(registry.source);
    fs::read_to_string(&path).map_err(|error| {
        SyncError(format!(
            "{}: cannot read {}: {error}",
            registry.name,
            path.display()
        ))
    })
}

fn source_labels(source: &str) -> Result<BTreeMap<i64, String>, SyncError> {
    let mut labels = BTreeMap::new();

    for line in source.lines() {
        let Some(const_start) = line.find("pub const ") else {
            continue;
        };
        let declaration = &line[const_start + "pub const ".len()..];
        let Some((name, value)) = declaration.split_once(": i64 =") else {
            continue;
        };
        let name = name.trim();
        let value = value.trim().trim_end_matches(';').trim();
        let Ok(label) = value.parse::<i64>() else {
            continue;
        };
        if labels.insert(label, name.to_owned()).is_some() {
            return Err(SyncError(format!("duplicate Rust label {label} in source")));
        }
    }

    Ok(labels)
}

fn differences(expected: &BTreeMap<i64, String>, actual: &BTreeMap<i64, String>) -> Vec<String> {
    let mut differences = Vec::new();

    for (label, name) in expected {
        if !actual.contains_key(label) {
            differences.push(format!("missing label {label} ({name})"));
        }
    }
    for (label, name) in actual {
        if !expected.contains_key(label) {
            differences.push(format!("stale label {label} ({name})"));
        }
    }

    differences
}

fn append_constants(
    source: &str,
    actual: &BTreeMap<i64, String>,
    missing: &[(i64, String)],
    registry: &Registry,
) -> Result<String, SyncError> {
    let module_start = source
        .find("pub mod label")
        .ok_or_else(|| SyncError(format!("{}: missing `pub mod label`", registry.name)))?;
    let module_end = source[module_start..]
        .rfind('}')
        .map(|offset| module_start + offset)
        .ok_or_else(|| SyncError(format!("{}: unterminated label module", registry.name)))?;
    let mut used_names: BTreeMap<String, i64> = actual
        .iter()
        .map(|(label, name)| (name.clone(), *label))
        .collect();
    let mut additions = String::new();

    for (label, description) in missing {
        let mut name = constant_name(description);
        if let Some(existing_label) = used_names.get(&name) {
            if existing_label != label {
                name = format!("{name}_LABEL_{label}");
            }
        }
        used_names.insert(name.clone(), *label);
        additions.push_str(&format!(
            "    /// {description}.\n    pub const {name}: i64 = {label};\n"
        ));
    }

    let mut updated = source.to_owned();
    updated.insert_str(module_end, &format!("\n{additions}"));
    Ok(updated)
}

fn constant_name(description: &str) -> String {
    let mut name = String::new();
    let mut separator = false;

    for character in description.chars() {
        if character.is_ascii_alphanumeric() {
            if separator && !name.is_empty() {
                name.push('_');
            }
            name.push(character.to_ascii_uppercase());
            separator = false;
        } else {
            separator = true;
        }
    }

    if name.is_empty() {
        "LABEL".to_owned()
    } else if name.as_bytes()[0].is_ascii_digit() {
        format!("LABEL_{name}")
    } else {
        name
    }
}

fn parse_csv(input: &str) -> Result<Vec<Vec<String>>, String> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut characters = input.chars().peekable();
    let mut quoted = false;

    while let Some(character) = characters.next() {
        if quoted {
            match character {
                '"' if characters.peek() == Some(&'"') => {
                    characters.next();
                    field.push('"');
                }
                '"' => quoted = false,
                _ => field.push(character),
            }
            continue;
        }

        match character {
            '"' if field.is_empty() => quoted = true,
            ',' => {
                row.push(std::mem::take(&mut field));
            }
            '\n' => {
                row.push(std::mem::take(&mut field));
                if !row.is_empty() {
                    rows.push(std::mem::take(&mut row));
                }
            }
            '\r' => {}
            _ => field.push(character),
        }
    }

    if quoted {
        return Err("unterminated quoted field".to_owned());
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_csv_fields_and_skips_ranges() {
        let csv =
            "Label,Name,Reference\n0,Unset Detection,[RFC]\n1,\"A, B\",[RFC]\n2-4,Unassigned,\n";
        let labels = expected_labels(
            csv,
            &Registry {
                name: "test",
                url: "https://example.invalid/test.csv",
                source: "test.rs",
            },
        )
        .expect("CSV should parse");

        assert_eq!(labels.get(&0), Some(&"Unset Detection".to_owned()));
        assert_eq!(labels.get(&1), Some(&"A, B".to_owned()));
        assert!(!labels.contains_key(&2));
    }

    #[test]
    fn parses_rust_label_constants() {
        let source = "pub mod label {\n    pub const FETCH: i64 = 21;\n}\n";
        let labels = source_labels(source).expect("source should parse");

        assert_eq!(labels.get(&21), Some(&"FETCH".to_owned()));
    }

    #[test]
    fn derives_constant_names() {
        assert_eq!(constant_name("Payload Fetch"), "PAYLOAD_FETCH");
        assert_eq!(constant_name("Result / MAC"), "RESULT_MAC");
        assert_eq!(constant_name("123"), "LABEL_123");
    }
}
