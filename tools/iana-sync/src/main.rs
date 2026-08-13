use std::{collections::BTreeMap, env, fmt, fs, path::Path};

fn line_indent(source: &str, absolute_start: usize) -> usize {
    let line_start = source[..absolute_start]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    source[line_start..absolute_start]
        .chars()
        .take_while(|character| *character == ' ')
        .count()
}

struct Registry {
    name: &'static str,
    url: &'static str,
    source: &'static str,
    snapshot: &'static str,
    label_column: usize,
    name_column: usize,
    integer: IntegerType,
    is_known: fn(i128) -> bool,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum IntegerType {
    I32,
    I64,
    U8,
    U64,
    I128,
}

impl IntegerType {
    const fn rust_name(self) -> &'static str {
        match self {
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::U8 => "u8",
            Self::U64 => "u64",
            Self::I128 => "i128",
        }
    }
}

const REGISTRIES: &[Registry] = &[
    Registry {
        name: "SUIT Envelope Elements",
        url: "https://www.iana.org/assignments/suit/suit-envelope-elements.csv",
        source: "iana-suit/src/envelope.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_envelope_is_known,
    },
    Registry {
        name: "SUIT Manifest Elements",
        url: "https://www.iana.org/assignments/suit/suit-manifest-elements.csv",
        source: "iana-suit/src/manifest.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_manifest_is_known,
    },
    Registry {
        name: "SUIT Common Elements",
        url: "https://www.iana.org/assignments/suit/suit-common-elements.csv",
        source: "iana-suit/src/common.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_common_is_known,
    },
    Registry {
        name: "SUIT Commands",
        url: "https://www.iana.org/assignments/suit/suit-commands.csv",
        source: "iana-suit/src/commands.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_commands_is_known,
    },
    Registry {
        name: "SUIT Parameters",
        url: "https://www.iana.org/assignments/suit/suit-parameters.csv",
        source: "iana-suit/src/parameters.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_parameters_is_known,
    },
    Registry {
        name: "SUIT Text Values",
        url: "https://www.iana.org/assignments/suit/suit-text-values.csv",
        source: "iana-suit/src/text.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_text_is_known,
    },
    Registry {
        name: "SUIT Component Text Values",
        url: "https://www.iana.org/assignments/suit/suit-component-text-values.csv",
        source: "iana-suit/src/component_text.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_component_text_is_known,
    },
    Registry {
        name: "SUIT Report Elements",
        url: "https://www.iana.org/assignments/suit/suit-report-elements.csv",
        source: "iana-suit/src/report.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_report_is_known,
    },
    Registry {
        name: "SUIT Record Elements",
        url: "https://www.iana.org/assignments/suit/suit-record-elements.csv",
        source: "iana-suit/src/record.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_record_is_known,
    },
    Registry {
        name: "SUIT Report Reasons",
        url: "https://www.iana.org/assignments/suit/suit-report-reasons.csv",
        source: "iana-suit/src/report_reasons.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_report_reasons_is_known,
    },
    Registry {
        name: "SUIT Capability Report Elements",
        url: "https://www.iana.org/assignments/suit/suit-capability-report-elements.csv",
        source: "iana-suit/src/capability_report.rs",
        snapshot: iana_suit::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I32,
        is_known: suit_capability_report_is_known,
    },
    Registry {
        name: "CBOR Simple Values",
        url: "https://www.iana.org/assignments/cbor-simple-values/simple.csv",
        source: "iana-cbor/src/simple_values.rs",
        snapshot: iana_cbor::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::U8,
        is_known: cbor_simple_values_is_known,
    },
    Registry {
        name: "CBOR Tags",
        url: "https://www.iana.org/assignments/cbor-tags/tags.csv",
        source: "iana-cbor/src/tags.rs",
        snapshot: iana_cbor::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 2,
        integer: IntegerType::U64,
        is_known: cbor_tags_is_known,
    },
    Registry {
        name: "CBOR Timescales",
        url: "https://www.iana.org/assignments/cbor-tags/timescales.csv",
        source: "iana-cbor/src/timescales.rs",
        snapshot: iana_cbor::IANA_SNAPSHOT,
        label_column: 1,
        name_column: 0,
        integer: IntegerType::U8,
        is_known: cbor_timescales_is_known,
    },
    Registry {
        name: "CBOR Time Tag Map Keys",
        url: "https://www.iana.org/assignments/cbor-tags/time-tag-map-keys.csv",
        source: "iana-cbor/src/time_tag_map_keys.rs",
        snapshot: iana_cbor::IANA_SNAPSHOT,
        label_column: 0,
        name_column: 1,
        integer: IntegerType::I128,
        is_known: cbor_time_tag_map_keys_is_known,
    },
];

fn suit_envelope_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::envelope::is_known)
}
fn suit_manifest_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::manifest::is_known)
}
fn suit_common_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::common::is_known)
}
fn suit_commands_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::commands::is_known)
}
fn suit_parameters_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::parameters::is_known)
}
fn suit_text_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::text::is_known)
}
fn suit_component_text_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::component_text::is_known)
}
fn suit_report_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::report::is_known)
}
fn suit_record_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::record::is_known)
}
fn suit_report_reasons_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::report_reasons::is_known)
}
fn suit_capability_report_is_known(label: i128) -> bool {
    i32::try_from(label).map_or(false, iana_suit::capability_report::is_known)
}
fn cbor_simple_values_is_known(label: i128) -> bool {
    u8::try_from(label).map_or(false, iana_cbor::simple_values::is_known)
}
fn cbor_tags_is_known(label: i128) -> bool {
    u64::try_from(label).map_or(false, iana_cbor::tags::is_known)
}
fn cbor_timescales_is_known(label: i128) -> bool {
    u8::try_from(label).map_or(false, iana_cbor::timescales::is_known)
}
fn cbor_time_tag_map_keys_is_known(label: i128) -> bool {
    iana_cbor::time_tag_map_keys::is_known(label)
}

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

fn check_registry(_root: &Path, registry: &Registry) -> Result<String, SyncError> {
    if let Some(last_modified) = fetch_last_modified_date(registry)? {
        if registry.snapshot >= last_modified.as_str() {
            return Ok(format!("snapshot current ({last_modified})"));
        }
    }

    let expected = expected_labels(&download(registry)?, registry)?;
    let missing: Vec<(i128, &str)> = expected
        .iter()
        .filter(|(label, _)| !(registry.is_known)(**label))
        .map(|(label, name)| (*label, name.as_str()))
        .collect();

    if missing.is_empty() {
        Ok(format!("ok ({} assigned labels)", expected.len()))
    } else {
        let differences: Vec<String> = missing
            .into_iter()
            .map(|(label, name)| format!("missing label {label} ({name})"))
            .collect();
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
    let actual = source_labels(&source, registry)?;
    let missing: Vec<(i128, String)> = expected
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

    let validation_missing: Vec<(i128, &str)> = expected
        .iter()
        .filter(|(label, _)| !(registry.is_known)(**label))
        .map(|(label, name)| (*label, name.as_str()))
        .collect();
    if !validation_missing.is_empty() {
        let details = validation_missing
            .into_iter()
            .map(|(label, name)| format!("missing label {label} ({name})"))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(SyncError(format!("{}:\n{}", registry.name, details)));
    }

    Ok(format!("added {} label(s)", missing.len()))
}

fn download(registry: &Registry) -> Result<String, SyncError> {
    let response = agent()
        .get(registry.url)
        .call()
        .map_err(|error| SyncError(format!("{}: IANA request failed: {error}", registry.name)))?;

    response.into_string().map_err(|error| {
        SyncError(format!(
            "{}: IANA response is not UTF-8: {error}",
            registry.name
        ))
    })
}

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(15))
        .build()
}

fn fetch_last_modified_date(registry: &Registry) -> Result<Option<String>, SyncError> {
    let response = agent()
        .head(registry.url)
        .call()
        .map_err(|error| SyncError(format!("{}: IANA request failed: {error}", registry.name)))?;
    let Some(header) = response.header("Last-Modified") else {
        println!("Last-Modified unavailable; checking full CSV");
        return Ok(None);
    };
    let Some(date) = parse_last_modified_date(header) else {
        println!("Last-Modified unparseable ({header}); checking full CSV");
        return Ok(None);
    };
    Ok(Some(date))
}

fn parse_last_modified_date(header: &str) -> Option<String> {
    let parts: Vec<&str> = header.split_whitespace().collect();
    let day = parts.get(1)?;
    let month = parts.get(2)?;
    let year = parts.get(3)?;
    let month_num = match *month {
        "Jan" => "01",
        "Feb" => "02",
        "Mar" => "03",
        "Apr" => "04",
        "May" => "05",
        "Jun" => "06",
        "Jul" => "07",
        "Aug" => "08",
        "Sep" => "09",
        "Oct" => "10",
        "Nov" => "11",
        "Dec" => "12",
        _ => return None,
    };
    let day_padded = if day.len() == 1 {
        format!("0{day}")
    } else {
        day.to_string()
    };
    Some(format!("{year}-{month_num}-{day_padded}"))
}

fn expected_labels(csv: &str, registry: &Registry) -> Result<BTreeMap<i128, String>, SyncError> {
    let rows = parse_csv(csv).map_err(|error| SyncError(format!("{}: {error}", registry.name)))?;
    let mut labels = BTreeMap::new();

    for row in rows.into_iter().skip(1) {
        let Some(raw_label) = row.get(registry.label_column) else {
            continue;
        };
        let Some(label) = parse_label(raw_label, registry.integer) else {
            continue;
        };
        let name = row
            .get(registry.name_column)
            .map(String::as_str)
            .unwrap_or("Unnamed")
            .trim();
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

fn source_labels(source: &str, registry: &Registry) -> Result<BTreeMap<i128, String>, SyncError> {
    let mut labels = BTreeMap::new();
    let mut remaining = source;
    let needle = "pub const ";

    while let Some(const_start) = remaining.find(needle) {
        let absolute_start = source.len() - remaining.len() + const_start;
        let line_indent = line_indent(source, absolute_start);
        let line_start = source[..absolute_start]
            .rfind('\n')
            .map_or(0, |index| index + 1);
        let line_prefix = &source[line_start..absolute_start];
        if !line_prefix.trim().is_empty() {
            remaining = &remaining[const_start + needle.len()..];
            continue;
        }
        if line_indent >= 4 {
            remaining = &remaining[const_start + needle.len()..];
            continue;
        }
        remaining = &remaining[const_start + needle.len()..];
        let Some(end) = remaining.find(';') else {
            break;
        };
        let declaration = &remaining[..end];
        remaining = &remaining[end + 1..];
        let Some((name, declaration)) = declaration.split_once(':') else {
            continue;
        };
        let name = name.trim();
        if !name.chars().all(|character| {
            character.is_ascii_uppercase() || character.is_ascii_digit() || character == '_'
        }) {
            continue;
        }
        let Some((rust_type, value)) = declaration.split_once('=') else {
            continue;
        };
        if rust_type.trim() != registry.integer.rust_name() {
            return Err(SyncError(format!(
                "{}: constant {name} has type {} but expected {}",
                registry.name,
                rust_type.trim(),
                registry.integer.rust_name()
            )));
        }
        let value = value.trim().trim_end_matches(';').trim();
        let Some(label) = parse_label(value, registry.integer) else {
            continue;
        };
        if labels.insert(label, name.to_owned()).is_some() {
            return Err(SyncError(format!("duplicate Rust label {label} in source")));
        }
    }

    Ok(labels)
}

fn append_constants(
    source: &str,
    actual: &BTreeMap<i128, String>,
    missing: &[(i128, String)],
    registry: &Registry,
) -> Result<String, SyncError> {
    let mut used_names: BTreeMap<String, i128> = actual
        .iter()
        .map(|(label, name)| (name.clone(), *label))
        .collect();
    let mut additions = String::new();

    for (label, description) in missing {
        let mut name = constant_name(registry, *label, description);
        let description = doc_description(description);
        if let Some(existing_label) = used_names.get(&name)
            && existing_label != label
        {
            name = format!("{name}_LABEL_{label}");
        }
        used_names.insert(name.clone(), *label);
        additions.push_str(&format!(
            "/// {description}.\npub const {name}: {} = {label};\n",
            registry.integer.rust_name()
        ));
    }

    let mut updated = source.to_owned();
    if !updated.ends_with('\n') {
        updated.push('\n');
    }
    updated.push('\n');
    updated.push_str(&additions);
    Ok(updated)
}

fn constant_name(registry: &Registry, label: i128, description: &str) -> String {
    match registry.name {
        "CBOR Tags" => match label {
            107 => return "SUIT_ENVELOPE".to_owned(),
            1070 => return "SUIT_MANIFEST".to_owned(),
            _ => return format!("TAG_{label}"),
        },
        "CBOR Simple Values" => match label {
            20 => return "FALSE".to_owned(),
            21 => return "TRUE".to_owned(),
            22 => return "NULL".to_owned(),
            23 => return "UNDEFINED".to_owned(),
            _ => return format!("SIMPLE_VALUE_{label}"),
        },
        _ => {}
    }

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

fn doc_description(description: &str) -> String {
    description
        .replace(['\r', '\n'], " ")
        .replace(
            "https://262.ecma-international.org/14.0/#sec-regexp-regular-expression-objects",
            "<https://262.ecma-international.org/14.0/#sec-regexp-regular-expression-objects>",
        )
        .replace('[', "\\[")
        .replace(']', "\\]")
}

fn parse_label(raw: &str, integer: IntegerType) -> Option<i128> {
    let value = raw.trim().parse::<i128>().ok()?;
    match integer {
        IntegerType::I32 if i32::try_from(value).is_err() => None,
        IntegerType::I64 if i64::try_from(value).is_err() => None,
        IntegerType::U8 if u8::try_from(value).is_err() => None,
        IntegerType::U64 if u64::try_from(value).is_err() => None,
        IntegerType::I128 => Some(value),
        IntegerType::I32 | IntegerType::I64 | IntegerType::U8 | IntegerType::U64 => Some(value),
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
    fn parses_csv_rows() {
        let rows = parse_csv(
            "Label,Name
1,Assigned
2,Unassigned
3,Another Value
",
        )
        .expect("csv should parse");
        assert_eq!(rows[0], vec!["Label", "Name"]);
        assert_eq!(rows[1], vec!["1", "Assigned"]);
        assert_eq!(rows[2], vec!["2", "Unassigned"]);
        assert_eq!(rows[3], vec!["3", "Another Value"]);
    }

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
                snapshot: "2026-02-17",
                label_column: 0,
                name_column: 1,
                integer: IntegerType::I32,
                is_known: |_| false,
            },
        )
        .expect("CSV should parse");

        assert_eq!(labels.get(&0), Some(&"Unset Detection".to_owned()));
        assert_eq!(labels.get(&1), Some(&"A, B".to_owned()));
        assert!(!labels.contains_key(&2));
    }

    #[test]
    fn parses_rust_label_constants() {
        let source = "pub const FETCH: i32 = 21;\n";
        let registry = Registry {
            name: "test",
            url: "https://example.invalid/test.csv",
            source: "test.rs",
            snapshot: "2026-02-17",
            label_column: 0,
            name_column: 1,
            integer: IntegerType::I32,
            is_known: |_| false,
        };
        let labels = source_labels(source, &registry).expect("source should parse");

        assert_eq!(labels.get(&21), Some(&"FETCH".to_owned()));
    }

    #[test]
    fn parses_wide_and_multiline_values() {
        let source = "pub const TAG: u64 =\n    18446744073709551615;\n";
        let registry = Registry {
            name: "test",
            url: "https://example.invalid/test.csv",
            source: "test.rs",
            snapshot: "2026-07-20",
            label_column: 0,
            name_column: 1,
            integer: IntegerType::U64,
            is_known: |_| false,
        };
        let labels = source_labels(source, &registry).expect("source should parse");

        assert_eq!(labels.get(&i128::from(u64::MAX)), Some(&"TAG".to_owned()));
    }

    #[test]
    fn parses_signed_time_map_keys() {
        assert_eq!(
            parse_label("-18446744073709551616", IntegerType::I128),
            Some(-18446744073709551616)
        );
        assert_eq!(parse_label("2-3", IntegerType::I128), None);
        assert_eq!(parse_label("-1", IntegerType::U64), None);
    }

    #[test]
    fn derives_constant_names() {
        let registry = Registry {
            name: "test",
            url: "https://example.invalid/test.csv",
            source: "test.rs",
            snapshot: "2026-02-17",
            label_column: 0,
            name_column: 1,
            integer: IntegerType::I32,
            is_known: |_| false,
        };
        assert_eq!(
            constant_name(&registry, 1, "Payload Fetch"),
            "PAYLOAD_FETCH"
        );
        assert_eq!(constant_name(&registry, 2, "Result / MAC"), "RESULT_MAC");
        assert_eq!(constant_name(&registry, 3, "123"), "LABEL_123");
    }

    #[test]
    fn parses_last_modified_header() {
        assert_eq!(
            parse_last_modified_date("Mon, 17 Feb 2026 00:00:00 GMT"),
            Some("2026-02-17".to_owned())
        );
        assert_eq!(parse_last_modified_date("bogus"), None);
    }
}
