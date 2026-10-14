// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

use mc_contracts::DescriptorEntryV1;
use mc_contracts::primitives::{
    Bytes, Currency, GitPath, GitRef, Hex, Hostname, I64Str, Identifier, Opt, Origin,
    PrimitiveError, SetError, SetOf, U64Str, Uuid,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::fmt::Debug;

fn round_trip<T: DeserializeOwned + Serialize>(projection: Value) {
    let parsed: T = serde_json::from_value(projection.clone()).expect("valid projection");
    assert_eq!(serde_json::to_value(parsed).expect("serialize"), projection);
}

fn rejects<T: DeserializeOwned + Debug>(projection: Value, reason: impl std::fmt::Display) {
    let error = serde_json::from_value::<T>(projection).expect_err("must reject");
    let reason = reason.to_string();
    assert!(
        error.to_string().contains(&reason),
        "{error}; expected {reason}"
    );
}

#[test]
fn u64_boundaries_round_trip() {
    for value in ["0", "1", "18446744073709551614", "18446744073709551615"] {
        round_trip::<U64Str>(json!(value));
    }
}

#[test]
fn u64_rejects_json_number() {
    rejects::<U64Str>(json!(1), "invalid type: integer");
}

#[test]
fn u64_rejects_leading_zero() {
    rejects::<U64Str>(json!("00"), PrimitiveError::DecimalSyntax);
    rejects::<U64Str>(json!("01"), PrimitiveError::DecimalSyntax);
}

#[test]
fn u64_rejects_plus_and_exponent() {
    for value in ["+1", "1e3", "1E3", "-1", "-0", " 1", "1 ", ""] {
        rejects::<U64Str>(json!(value), PrimitiveError::DecimalSyntax);
    }
}

#[test]
fn u64_rejects_overflow() {
    rejects::<U64Str>(
        json!("18446744073709551616"),
        PrimitiveError::DecimalOverflow,
    );
}

#[test]
fn i64_boundaries_round_trip() {
    for value in [
        "-9223372036854775808",
        "-9223372036854775807",
        "-1",
        "0",
        "1",
        "9223372036854775806",
        "9223372036854775807",
    ] {
        round_trip::<I64Str>(json!(value));
    }
}

#[test]
fn i64_rejects_noncanonical_and_overflow() {
    rejects::<I64Str>(json!(-1), "invalid type: integer");
    for value in ["-0", "00", "-01", "+1", "1e3", " 1", "1 ", ""] {
        rejects::<I64Str>(json!(value), PrimitiveError::DecimalSyntax);
    }
    for value in ["-9223372036854775809", "9223372036854775808"] {
        rejects::<I64Str>(json!(value), PrimitiveError::DecimalOverflow);
    }
}

#[test]
fn hex_round_trip() {
    round_trip::<Hex<2>>(json!("00af"));
    round_trip::<Hex<32>>(json!("ab".repeat(32)));
    round_trip::<Hex<64>>(json!("cd".repeat(64)));
}

#[test]
fn hex_rejects_uppercase() {
    rejects::<Hex<2>>(json!("00Af"), PrimitiveError::HexSyntax);
}

#[test]
fn hex_rejects_wrong_length() {
    for value in ["", "00", "00a", "00aff", "00affe"] {
        rejects::<Hex<2>>(json!(value), PrimitiveError::HexLength);
    }
}

#[test]
fn bytes_enforce_even_lowercase_hex_and_size_boundary() {
    round_trip::<Bytes>(json!(""));
    round_trip::<Bytes>(json!("00ff"));
    // REGISTRY.json bytes.maxLength is 2,097,152 hexadecimal characters.
    for size in [1_048_575, 1_048_576] {
        round_trip::<Bytes>(json!("ab".repeat(size)));
    }
    rejects::<Bytes>(json!("ab".repeat(1_048_577)), PrimitiveError::ByteLimit);
    for value in ["0", "0g", "AF"] {
        rejects::<Bytes>(json!(value), PrimitiveError::HexSyntax);
    }
    let oversized = Bytes(vec![0; 1_048_577]);
    assert!(
        serde_json::to_value(oversized)
            .expect_err("size guard")
            .to_string()
            .contains(&PrimitiveError::ByteLimit.to_string())
    );
}

#[test]
fn uuid_round_trip_and_rejects_noncanonical_shape() {
    round_trip::<Uuid>(json!("00112233-4455-6677-8899-aabbccddeeff"));
    for value in [
        "00112233445566778899aabbccddeeff",
        "00112233_4455-6677-8899-aabbccddeeff",
    ] {
        rejects::<Uuid>(json!(value), PrimitiveError::UuidSyntax);
    }
}

#[test]
fn uuid_rejects_uppercase() {
    rejects::<Uuid>(
        json!("00112233-4455-6677-8899-AABBCCDDEEFF"),
        PrimitiveError::UuidSyntax,
    );
}

#[test]
fn git_path_round_trip_preserves_raw_bytes_and_size_boundary() {
    round_trip::<GitPath>(json!("6469722f66696c65"));
    // Git paths are raw bytes: invalid UTF-8 is not normalized or rejected.
    round_trip::<GitPath>(json!("ff"));
    for size in [4095, 4096] {
        round_trip::<GitPath>(json!("61".repeat(size)));
    }
    rejects::<GitPath>(json!("61".repeat(4097)), PrimitiveError::ByteLimit);
}

#[test]
fn git_path_rejects_forbidden_components() {
    for bytes in [
        b"".as_slice(),
        b"/a",
        b"a/",
        b"a//b",
        b".",
        b"..",
        b"a/./b",
        b"a/../b",
        b"a\\b",
        b"a\0b",
    ] {
        assert!(matches!(
            GitPath::new(bytes.to_vec()),
            Err(PrimitiveError::GitPathSyntax)
        ));
    }
    assert!(
        serde_json::to_value(GitPath(b"../a".to_vec()))
            .expect_err("path guard")
            .to_string()
            .contains(&PrimitiveError::GitPathSyntax.to_string())
    );
}

#[test]
fn identifier_enforces_lexical_length_and_nfc_rules() {
    round_trip::<Identifier>(json!("A0._:/@+ -"));
    for size in [1, 255, 256] {
        round_trip::<Identifier>(json!("a".repeat(size)));
    }
    rejects::<Identifier>(json!(""), PrimitiveError::TextLength);
    rejects::<Identifier>(json!("a".repeat(257)), PrimitiveError::TextLength);
    for value in ["_a", "a?", "é"] {
        rejects::<Identifier>(json!(value), PrimitiveError::IdentifierSyntax);
    }
    rejects::<Identifier>(json!("a-e\u{301}"), PrimitiveError::NonNfc);
    assert!(
        serde_json::to_value(Identifier("a?".to_owned()))
            .expect_err("lexical guard")
            .to_string()
            .contains(&PrimitiveError::IdentifierSyntax.to_string())
    );
}

#[test]
fn git_ref_accepts_nfc_unicode_and_enforces_byte_limit() {
    round_trip::<GitRef>(json!("refs/heads/é"));
    for size in [1012, 1013] {
        round_trip::<GitRef>(json!(format!("refs/heads/{}", "a".repeat(size))));
    }
    rejects::<GitRef>(
        json!(format!("refs/heads/{}", "a".repeat(1014))),
        PrimitiveError::TextLength,
    );
    round_trip::<GitRef>(json!(format!("refs/heads/a{}", "é".repeat(506))));
    rejects::<GitRef>(
        json!(format!("refs/heads/a{}", "é".repeat(507))),
        PrimitiveError::TextLength,
    );
    rejects::<GitRef>(json!("refs/heads/e\u{301}"), PrimitiveError::NonNfc);
}

#[test]
fn git_ref_rejects_forbidden_grammar_without_fixup() {
    for value in [
        "refs/tags/main",
        "refs/heads/a..b",
        "refs/heads/a@{b",
        "refs/heads/a\\b",
        "refs/heads/a//b",
        "refs/heads/a/",
        "refs/heads/a.",
        "refs/heads/.a",
        "refs/heads/a/.b",
        "refs/heads/a.lock",
        "refs/heads/a.lock/b",
        "refs/heads/a b",
        "refs/heads/a~b",
        "refs/heads/a^b",
        "refs/heads/a:b",
        "refs/heads/a?b",
        "refs/heads/a*b",
        "refs/heads/a[b",
    ] {
        rejects::<GitRef>(json!(value), PrimitiveError::GitRefSyntax);
    }
}

#[test]
fn hostname_enforces_registry_lexical_and_total_limits() {
    round_trip::<Hostname>(json!("example.test"));
    // Standalone hostname grammar is [a-z0-9.-]+; Origin adds host-label rules.
    round_trip::<Hostname>(json!("-a..test."));
    for length in [1, 64, 252, 253] {
        round_trip::<Hostname>(json!("a".repeat(length)));
    }
    rejects::<Hostname>(json!(""), PrimitiveError::TextLength);
    rejects::<Hostname>(json!("a".repeat(254)), PrimitiveError::TextLength);
    for value in ["Example.test", "a_test", "a/test", "a test"] {
        rejects::<Hostname>(json!(value), PrimitiveError::HostnameSyntax);
    }
}

#[test]
fn origin_accepts_exact_https_and_rejects_alternate_spellings() {
    round_trip::<Origin>(json!("https://example.test"));
    round_trip::<Origin>(json!("https://example.test:8443"));
    round_trip::<Origin>(json!("https://example.test:0"));
    round_trip::<Origin>(json!("https://example.test:08443"));
    round_trip::<Origin>(json!("https://example.test:65535"));
    for value in [
        "http://example.test",
        "https://Example.test",
        "https://example.test:443",
        "https://example.test:0443",
        "https://example.test:65536",
        "https://example.test:",
        "https://example.test/",
        "https://user@example.test",
        "https://example.test?a",
        "https://example.test#a",
        "https://a..test",
        "https://-a.test",
        "https://a-.test",
    ] {
        rejects::<Origin>(json!(value), PrimitiveError::OriginSyntax);
    }
}

#[test]
fn currency_requires_three_uppercase_ascii_letters() {
    round_trip::<Currency>(json!("USD"));
    for value in ["usd", "U1D"] {
        rejects::<Currency>(json!(value), PrimitiveError::CurrencySyntax);
    }
    for value in ["US", "USDD", "ÜSD"] {
        rejects::<Currency>(json!(value), PrimitiveError::TextLength);
    }
}

#[test]
fn opt_round_trip_distinguishes_absent_from_empty_set() {
    round_trip::<Opt<U64Str>>(json!([]));
    round_trip::<Opt<U64Str>>(json!(["0"]));
    round_trip::<Opt<SetOf<U64Str>>>(json!([[]]));
    rejects::<Opt<U64Str>>(json!(null), "invalid type: null");
}

#[test]
fn opt_rejects_two_elements() {
    rejects::<Opt<U64Str>>(
        json!(["1", "2"]),
        "option array contains more than one element",
    );
}

#[test]
fn set_round_trip_and_constructor_preserve_input_order() {
    round_trip::<SetOf<U64Str>>(json!([]));
    round_trip::<SetOf<U64Str>>(json!(["0", "1", "23", "24", "255", "256"]));
    let set = SetOf::new(vec![U64Str(2), U64Str(10)]).expect("canonical order");
    assert_eq!(set.as_slice(), &[U64Str(2), U64Str(10)]);
    assert_eq!(set.into_vec(), vec![U64Str(2), U64Str(10)]);
}

#[test]
fn set_rejects_unsorted() {
    rejects::<SetOf<U64Str>>(json!(["2", "1"]), SetError::Unsorted);
    assert!(matches!(
        SetOf::new(vec![U64Str(2), U64Str(1)]),
        Err(SetError::Unsorted)
    ));
}

#[test]
fn set_rejects_duplicate() {
    rejects::<SetOf<U64Str>>(json!(["1", "1"]), SetError::Duplicate);
    assert!(matches!(
        SetOf::new(vec![U64Str(1), U64Str(1)]),
        Err(SetError::Duplicate)
    ));
}

#[test]
fn set_uses_encoded_text_length_before_content() {
    // ENCODING.md: complete encoded bytes put 0x61,b before 0x62,a,a.
    round_trip::<SetOf<Identifier>>(json!(["b", "aa"]));
    rejects::<SetOf<Identifier>>(json!(["aa", "b"]), SetError::Unsorted);
    round_trip::<SetOf<Bytes>>(json!(["ff", "0000"]));
    rejects::<SetOf<Bytes>>(json!(["0000", "ff"]), SetError::Unsorted);
}

#[test]
fn set_uses_integer_encoding_instead_of_decimal_or_signed_order() {
    round_trip::<SetOf<U64Str>>(json!(["2", "10"]));
    rejects::<SetOf<U64Str>>(json!(["10", "2"]), SetError::Unsorted);
    round_trip::<SetOf<I64Str>>(json!(["0", "1", "-1", "-2"]));
    rejects::<SetOf<I64Str>>(json!(["-2", "-1"]), SetError::Unsorted);
    rejects::<SetOf<I64Str>>(json!(["-1", "0"]), SetError::Unsorted);
}

#[test]
fn set_compares_option_and_array_headers_before_elements() {
    round_trip::<SetOf<Opt<U64Str>>>(json!([[], ["0"], ["1"]]));
    rejects::<SetOf<Opt<U64Str>>>(json!([["0"], []]), SetError::Unsorted);
    round_trip::<SetOf<Vec<U64Str>>>(json!([["23"], ["0", "0"]]));
    rejects::<SetOf<Vec<U64Str>>>(json!([["0", "0"], ["23"]]), SetError::Unsorted);
    round_trip::<SetOf<Opt<Vec<U64Str>>>>(json!([[], [["23"]], [["0", "0"]]]));
    rejects::<SetOf<Opt<Vec<U64Str>>>>(json!([[["0", "0"]], [["23"]]]), SetError::Unsorted);
}

#[test]
fn set_compares_nested_record_fields_in_registry_order() {
    let short_key = json!({"type":"DescriptorEntryV1", "wire_revision":1, "key":"b", "value":"ff"});
    let long_key = json!({"type":"DescriptorEntryV1", "wire_revision":1, "key":"aa", "value":"00"});
    round_trip::<SetOf<DescriptorEntryV1>>(json!([short_key, long_key]));
    rejects::<SetOf<DescriptorEntryV1>>(json!([long_key, short_key]), SetError::Unsorted);
    // Equal first fields defer to the byte-string header of the next field.
    let short_value =
        json!({"type":"DescriptorEntryV1", "wire_revision":1, "key":"a", "value":"ff"});
    let long_value =
        json!({"type":"DescriptorEntryV1", "wire_revision":1, "key":"a", "value":"0000"});
    round_trip::<SetOf<DescriptorEntryV1>>(json!([short_value, long_value]));
    rejects::<SetOf<DescriptorEntryV1>>(json!([long_value, short_value]), SetError::Unsorted);
}

#[test]
fn set_enforces_item_count_boundary() {
    // ENCODING.md: maximum array length is 65,536.
    for size in [65_535, 65_536] {
        let set = SetOf::new((0..size).map(U64Str).collect()).expect("within limit");
        assert_eq!(set.as_slice().len(), size as usize);
    }
    assert!(matches!(
        SetOf::new((0..65_537).map(U64Str).collect()),
        Err(SetError::TooMany)
    ));
}
