//! PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
//! Generate closed JSON projection types from the frozen wire registry.

use clap::Parser;
use serde_json::{Map, Value, json};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// ENCODING.md §Registry identity: exact distributed bytes, not reserialized JSON.
const FROZEN_REGISTRY: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../ARCHITECTURE/V2_1/SCHEMAS/REGISTRY.json"
));
// coordination/AGENTS.md §1: frozen V2.1 registry identity.
const REGISTRY_SHA256: &str = "b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787";

#[derive(Parser)]
#[command(about = "Generate frozen Mastercoder JSON projection contracts")]
struct Args {
    #[arg(long)]
    registry: Option<PathBuf>,
    #[arg(long)]
    out: Option<PathBuf>,
    /// Verify freshness without writing output.
    #[arg(long)]
    check: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let registry_path = args
        .registry
        .unwrap_or_else(|| manifest.join("../../../ARCHITECTURE/V2_1/SCHEMAS/REGISTRY.json"));
    let out = args
        .out
        .unwrap_or_else(|| manifest.join("../../crates/mc-contracts/src/generated.rs"));
    let bytes = fs::read(&registry_path)?;
    if bytes != FROZEN_REGISTRY {
        return Err("registry differs from the exact frozen V2.1 registry bytes".into());
    }
    let registry: Value = serde_json::from_slice(&bytes)?;
    validate_schemas(&registry, &registry_path)?;
    let generated = format_source(&generate(&registry)?)?;
    if args.check {
        let existing = fs::read(&out)?;
        if existing != generated {
            return Err(format!("generated contracts are stale: {}", out.display()).into());
        }
    } else {
        fs::write(&out, generated)?;
    }
    Ok(())
}

fn object(value: &Value) -> Result<&Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| "expected registry object".into())
}

fn array(value: &Value) -> Result<&[Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "expected registry array".into())
}

fn string(value: &Value) -> Result<&str> {
    value
        .as_str()
        .ok_or_else(|| "expected registry string".into())
}

fn field_names(contract: &Value) -> Result<Vec<&str>> {
    let mut names = vec!["type", "wire_revision"];
    for field in array(&contract["fields"])? {
        names.push(string(&field["name"])?);
    }
    Ok(names)
}

fn projection_schema(ty: &Value, registry: &Value) -> Result<Value> {
    if let Some(name) = ty.as_str() {
        if let Some(primitive) = object(&registry["primitives"])?.get(name) {
            return Ok(primitive.clone());
        }
        if object(&registry["contracts"])?.contains_key(name) {
            return Ok(json!({"$ref": format!("{name}.schema.json")}));
        }
        return Err(format!("unregistered field type: {name}").into());
    }
    let descriptor = object(ty)?;
    if let Some(values) = descriptor.get("enum") {
        let mut codes = Map::new();
        for (index, value) in array(values)?.iter().enumerate() {
            codes.insert(string(value)?.to_owned(), json!(index));
        }
        return Ok(json!({"enum": values, "x-cbor-enum-codes": codes}));
    }
    for key in ["option", "set", "array"] {
        if let Some(inner) = descriptor.get(key) {
            let mut schema = json!({"type": "array", "items": projection_schema(inner, registry)?});
            if key == "option" {
                schema["minItems"] = json!(0);
                schema["maxItems"] = json!(1);
            } else {
                schema["maxItems"] = descriptor
                    .get("max_items")
                    .ok_or("missing registry array bound")?
                    .clone();
            }
            if key == "set" {
                schema["uniqueItems"] = json!(true);
                schema["x-order"] = json!("lexicographic canonical encoded element bytes");
            }
            return Ok(schema);
        }
    }
    Err("unsupported registry field descriptor".into())
}

fn validate_schemas(registry: &Value, registry_path: &Path) -> Result<()> {
    let directory = registry_path
        .parent()
        .ok_or("registry path has no parent")?;
    for (name, contract) in object(&registry["contracts"])? {
        let schema: Value =
            serde_json::from_slice(&fs::read(directory.join(format!("{name}.schema.json")))?)?;
        let names = field_names(contract)?;
        let properties = object(&schema["properties"])?;
        if schema["type"] != "object"
            || schema["title"] != *name
            || schema["additionalProperties"] != false
            || schema["required"] != json!(names)
            || schema["x-cbor-order"] != json!(names)
            || properties.len() != names.len()
            || schema["properties"]["type"] != json!({"const":name})
            || schema["properties"]["wire_revision"] != json!({"const":contract["wire_revision"]})
            || contract["wire_revision"] != 1
        {
            return Err(format!("schema header/order disagrees with registry: {name}").into());
        }
        for field in array(&contract["fields"])? {
            let field_name = string(&field["name"])?;
            if properties.get(field_name) != Some(&projection_schema(&field["type"], registry)?) {
                return Err(
                    format!("schema field disagrees with registry: {name}.{field_name}").into(),
                );
            }
        }
    }
    Ok(())
}

fn pascal(name: &str) -> Result<String> {
    let mut result = String::new();
    for part in name.split('_') {
        let mut characters = part.chars();
        let first = characters.next().ok_or("empty Rust name component")?;
        if !first.is_ascii_alphanumeric() || !characters.clone().all(|c| c.is_ascii_alphanumeric())
        {
            return Err(format!("unsupported Rust name: {name}").into());
        }
        result.push(first.to_ascii_uppercase());
        result.extend(characters.map(|c| c.to_ascii_lowercase()));
    }
    Ok(result)
}

fn emit_string_deserialize(
    out: &mut String,
    name: &str,
    variants: &[(&str, String)],
) -> Result<()> {
    // JSON schemas admit literal strings, never Serde's externally tagged unit maps.
    writeln!(
        out,
        "impl<'de> Deserialize<'de> for {name} {{\nfn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{\nlet value = String::deserialize(deserializer)?;\nmatch value.as_str() {{"
    )?;
    for (literal, variant) in variants {
        writeln!(out, "{literal:?} => Ok(Self::{variant}),")?;
    }
    let literals: Vec<_> = variants.iter().map(|(literal, _)| *literal).collect();
    writeln!(
        out,
        "_ => Err(serde::de::Error::unknown_variant(&value, &{literals:?})),\n}}\n}}\n}}"
    )?;
    Ok(())
}

fn emit_enum(out: &mut String, name: &str, values: &[Value]) -> Result<()> {
    writeln!(
        out,
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]\npub enum {name} {{"
    )?;
    let mut variants = BTreeSet::new();
    let mut literal_variants = Vec::new();
    for value in values {
        let literal = string(value)?;
        let variant = pascal(literal)?;
        if !variants.insert(variant.clone()) {
            return Err(format!("Rust enum variant collision: {name}.{variant}").into());
        }
        writeln!(out, "#[serde(rename = {literal:?})]\n{variant},")?;
        literal_variants.push((literal, variant));
    }
    writeln!(out, "}}")?;
    emit_string_deserialize(out, name, &literal_variants)?;
    writeln!(
        out,
        "impl {name} {{\npub fn code(&self) -> u64 {{ match self {{"
    )?;
    for (index, value) in values.iter().enumerate() {
        writeln!(out, "Self::{} => {index},", pascal(string(value)?)?)?;
    }
    writeln!(
        out,
        "}} }}\npub fn from_code(code: u64) -> Option<Self> {{ match code {{"
    )?;
    for (index, value) in values.iter().enumerate() {
        writeln!(out, "{index} => Some(Self::{}),", pascal(string(value)?)?)?;
    }
    writeln!(
        out,
        "_ => None, }} }}\n}}\nimpl CanonicalOrder for {name} {{\nfn canonical_cmp(&self, other: &Self) -> Ordering {{ self.code().cmp(&other.code()) }}\nfn canonical_size(&self) -> usize {{ crate::primitives::canonical_header_len(self.code()) }}\nfn canonical_nesting(&self) -> usize {{ 0 }}\n}}"
    )?;
    Ok(())
}

fn rust_type(ty: &Value, context: &str, registry: &Value, out: &mut String) -> Result<String> {
    if let Some(name) = ty.as_str() {
        let mapped = match name {
            "bool" | "u16" | "i16" | "u32" => name,
            "u64" => "U64Str",
            "i64" => "I64Str",
            "uuid" => "Uuid",
            "digest" | "bytes32" => "Hex<32>",
            "bytes64" => "Hex<64>",
            "bytes" => "Bytes",
            "gitpath" => "GitPath",
            "identifier" => "Identifier",
            "ref" => "GitRef",
            "hostname" => "Hostname",
            "origin" => "Origin",
            "currency" => "Currency",
            "provider" => "Provider",
            "effect_kind" => "EffectKind",
            other if object(&registry["contracts"])?.contains_key(other) => other,
            _ => return Err(format!("unsupported primitive: {name}").into()),
        };
        return Ok(mapped.to_owned());
    }
    let descriptor = object(ty)?;
    if let Some(values) = descriptor.get("enum") {
        emit_enum(out, context, array(values)?)?;
        return Ok(context.to_owned());
    }
    for (key, wrapper) in [("option", "Opt"), ("set", "SetOf"), ("array", "Vec")] {
        if let Some(inner) = descriptor.get(key) {
            if key != "option" && descriptor["max_items"] != 65_536 {
                return Err("unsupported narrower collection bound".into());
            }
            return Ok(format!(
                "{wrapper}<{}>",
                rust_type(inner, context, registry, out)?
            ));
        }
    }
    Err("unsupported registry type descriptor".into())
}

fn generate(registry: &Value) -> Result<String> {
    let mut out = String::from(
        "// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)\n// Generated from SCHEMAS/REGISTRY.json; do not edit by hand.\nuse crate::primitives::*;\nuse serde::{Serialize, Deserialize};\nuse serde::ser::SerializeStruct;\nuse std::cmp::Ordering;\n",
    );
    writeln!(
        out,
        "// ENCODING.md §Registry identity: SHA-256 of the frozen raw registry bytes.\npub const REGISTRY_SHA256: &str = {REGISTRY_SHA256:?};"
    )?;
    for (name, primitive) in object(&registry["primitives"])? {
        if let Some(values) = primitive.get("enum") {
            emit_enum(&mut out, &pascal(name)?, array(values)?)?;
        }
    }
    let contracts = object(&registry["contracts"])?;
    for (name, contract) in contracts {
        let mut fields = Vec::new();
        for field in array(&contract["fields"])? {
            let field_name = string(&field["name"])?;
            let context = format!("{name}{}", pascal(field_name)?);
            fields.push((
                field_name,
                rust_type(&field["type"], &context, registry, &mut out)?,
                &field["type"],
            ));
        }
        writeln!(
            out,
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]\npub enum {name}Type {{ #[serde(rename = {name:?})] Value }}"
        )?;
        emit_string_deserialize(
            &mut out,
            &format!("{name}Type"),
            &[(name, "Value".to_owned())],
        )?;
        writeln!(
            out,
            "#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]\n#[serde(deny_unknown_fields, try_from = \"crate::primitives::ObjectOnly<{name}Fields>\")]\npub struct {name} {{\npub r#type: {name}Type,\npub wire_revision: WireRevision,"
        )?;
        for (field, ty, _) in &fields {
            writeln!(out, "pub {field}: {ty},")?;
        }
        writeln!(
            out,
            "}}\n#[derive(Deserialize)]\n#[serde(deny_unknown_fields)]\nstruct {name}Fields {{\nr#type: {name}Type,\nwire_revision: WireRevision,"
        )?;
        for (field, ty, descriptor) in &fields {
            if descriptor.get("array").is_some() {
                writeln!(
                    out,
                    "#[serde(deserialize_with = \"crate::primitives::bounded_array\")]"
                )?;
            }
            writeln!(out, "{field}: {ty},")?;
        }
        writeln!(
            out,
            "}}\nimpl TryFrom<crate::primitives::ObjectOnly<{name}Fields>> for {name} {{\ntype Error = ProjectionError;\nfn try_from(input: crate::primitives::ObjectOnly<{name}Fields>) -> Result<Self, Self::Error> {{\nlet fields = input.0;\nlet value = Self {{ r#type: fields.r#type, wire_revision: fields.wire_revision,"
        )?;
        for (field, _, _) in &fields {
            writeln!(out, "{field}: fields.{field},")?;
        }
        writeln!(
            out,
            "}};\nvalue.validate_projection_limits()?;\nOk(value)\n}}\n}}\nimpl Serialize for {name} {{\nfn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {{\nself.validate_projection_limits().map_err(serde::ser::Error::custom)?;\nlet mut state = serializer.serialize_struct(Self::TYPE_NAME, {})?;\nstate.serialize_field(\"type\", &self.r#type)?;\nstate.serialize_field(\"wire_revision\", &self.wire_revision)?;",
            fields.len() + 2
        )?;
        for (field, _, descriptor) in &fields {
            if descriptor.get("array").is_some() {
                writeln!(
                    out,
                    "state.serialize_field({field:?}, &crate::primitives::BoundedArrayRef(&self.{field}))?;"
                )?;
            } else {
                writeln!(out, "state.serialize_field({field:?}, &self.{field})?;")?;
            }
        }
        writeln!(out, "state.end()\n}}\n}}")?;
        writeln!(
            out,
            "impl {name} {{\npub const TYPE_NAME: &'static str = {name:?};\npub const FIELD_ORDER: &'static [&'static str] = &{:?};\npub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {{ crate::primitives::validate_projection_limits(self.canonical_size(), self.canonical_nesting()) }}\n}}\nimpl CanonicalOrder for {name} {{\nfn canonical_cmp(&self, other: &Self) -> Ordering {{",
            field_names(contract)?
        )?;
        // ENCODING.md §Exact bytes: same-type record headers have identical bytes.
        writeln!(out, "Ordering::Equal")?;
        for (field, _, _) in &fields {
            writeln!(
                out,
                ".then_with(|| self.{field}.canonical_cmp(&other.{field}))"
            )?;
        }
        writeln!(out, "}}")?;
        // ENCODING.md §Exact bytes: record array, exact type text, revision, then fields.
        writeln!(
            out,
            "fn canonical_size(&self) -> usize {{\ncrate::primitives::canonical_header_len({}u64)\n.saturating_add(crate::primitives::canonical_header_len(Self::TYPE_NAME.len() as u64))\n.saturating_add(Self::TYPE_NAME.len())\n.saturating_add(1)",
            fields.len() + 2
        )?;
        for (field, _, _) in &fields {
            writeln!(out, ".saturating_add(self.{field}.canonical_size())")?;
        }
        writeln!(out, "}}\nfn canonical_nesting(&self) -> usize {{")?;
        let Some((first, _, _)) = fields.first() else {
            return Err(format!("registered record has no fields: {name}").into());
        };
        writeln!(
            out,
            "1usize.saturating_add(self.{first}.canonical_nesting()"
        )?;
        for (field, _, _) in fields.iter().skip(1) {
            writeln!(out, ".max(self.{field}.canonical_nesting())")?;
        }
        writeln!(out, ")\n}}")?;
        for (field, _, descriptor) in &fields {
            if ["unit_id", "path", "tool_id", "step_id"].contains(field) {
                match descriptor.as_str() {
                    Some("identifier") => writeln!(
                        out,
                        "fn identity_key(&self) -> Option<Vec<u8>> {{ Some(self.{field}.0.as_bytes().to_vec()) }}"
                    )?,
                    Some("gitpath") => writeln!(
                        out,
                        "fn identity_key(&self) -> Option<Vec<u8>> {{ Some(self.{field}.0.clone()) }}"
                    )?,
                    // ModelContentBlockV1.tool_id is an optional reference, not an identity.
                    None if descriptor.get("option").is_some() => {}
                    _ => return Err(format!("unsupported identity field: {name}.{field}").into()),
                }
            }
        }
        writeln!(out, "}}")?;
    }
    writeln!(
        out,
        "pub const ALL_TYPE_NAMES: &[&str] = &{:?};\npub fn field_order(type_name: &str) -> Option<&'static [&'static str]> {{ match type_name {{",
        contracts.keys().collect::<Vec<_>>()
    )?;
    for name in contracts.keys() {
        writeln!(out, "{name:?} => Some({name}::FIELD_ORDER),")?;
    }
    writeln!(
        out,
        "_ => None, }} }}\npub fn round_trip_projection(value: serde_json::Value) -> Result<serde_json::Value, serde_json::Error> {{\nlet name = value.get(\"type\").and_then(serde_json::Value::as_str).ok_or_else(|| <serde_json::Error as serde::de::Error>::custom(\"missing or invalid contract type\"))?.to_owned();\nmatch name.as_str() {{"
    )?;
    for name in contracts.keys() {
        writeln!(
            out,
            "{name:?} => serde_json::to_value(serde_json::from_value::<{name}>(value)?),"
        )?;
    }
    writeln!(
        out,
        "_ => Err(<serde_json::Error as serde::de::Error>::custom(\"unregistered contract type\")), }} }}"
    )?;
    Ok(out)
}

fn format_source(source: &str) -> Result<Vec<u8>> {
    // Format in memory so --check never mutates generated.rs or scratch files.
    let mut child = Command::new("rustfmt")
        .args(["--emit", "stdout", "--edition", "2024"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or("rustfmt stdin unavailable")?
        .write_all(source.as_bytes())?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(format!(
            "rustfmt rejected generated source: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(output.stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_schema_projection_matches_registry() -> Result<()> {
        let registry = serde_json::from_slice(FROZEN_REGISTRY)?;
        validate_schemas(
            &registry,
            &Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../ARCHITECTURE/V2_1/SCHEMAS/REGISTRY.json"),
        )
    }

    #[test]
    fn generator_is_deterministic_and_closed() -> Result<()> {
        let registry = serde_json::from_slice(FROZEN_REGISTRY)?;
        let first = generate(&registry)?;
        assert_eq!(first, generate(&registry)?);
        let count = object(&registry["contracts"])?.len();
        assert_eq!(
            first.matches("#[serde(deny_unknown_fields").count(),
            count * 2
        );
        assert_eq!(first.matches("pub r#type:").count(), count);
        assert!(!first.contains("Default"));
        assert_eq!(pascal("SHA256")?, "Sha256");
        assert_eq!(pascal("CREATE_FIX_PR")?, "CreateFixPr");
        Ok(())
    }

    #[test]
    fn committed_output_is_byte_identical() -> Result<()> {
        let registry = serde_json::from_slice(FROZEN_REGISTRY)?;
        let expected = format_source(&generate(&registry)?)?;
        let committed = fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../crates/mc-contracts/src/generated.rs"),
        )?;
        assert_eq!(committed, expected, "regenerate the committed contracts");
        Ok(())
    }

    #[test]
    fn schema_preserves_field_specific_enum_subset() -> Result<()> {
        let registry: Value = serde_json::from_slice(FROZEN_REGISTRY)?;
        let expected = projection_schema(
            &registry["contracts"]["GitPublicationCapabilityV2"]["fields"]
                .as_array()
                .ok_or("missing capability fields")?
                .iter()
                .find(|field| field["name"] == "effect_kind")
                .ok_or("missing effect_kind field")?["type"],
            &registry,
        )?;
        assert_ne!(
            expected["enum"],
            registry["primitives"]["effect_kind"]["enum"]
        );
        assert_eq!(expected["x-cbor-enum-codes"]["CREATE_FIX_PR"], 4);
        Ok(())
    }
}
