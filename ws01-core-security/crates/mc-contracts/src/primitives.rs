// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
//! Strict JSON projections of the primitives in SCHEMAS/REGISTRY.json.

use serde::de::{self, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use unicode_normalization::UnicodeNormalization;

// ENCODING.md §Exact bytes; registry primitives bytes/gitpath fix narrower bounds.
const MAX_ARRAY_LEN: usize = 65_536;
const MAX_BYTES_LEN: usize = 1_048_576;
const MAX_GIT_PATH_LEN: usize = 4_096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PrimitiveError {
    #[error("noncanonical decimal syntax")]
    DecimalSyntax,
    #[error("decimal overflow")]
    DecimalOverflow,
    #[error("hexadecimal must be lowercase and even length")]
    HexSyntax,
    #[error("incorrect hexadecimal length")]
    HexLength,
    #[error("byte length exceeds registry limit")]
    ByteLimit,
    #[error("UUID must be lowercase hyphenated hexadecimal")]
    UuidSyntax,
    #[error("text must be Unicode NFC")]
    NonNfc,
    #[error("text length violates registry limits")]
    TextLength,
    #[error("invalid identifier syntax")]
    IdentifierSyntax,
    #[error("invalid Git ref syntax")]
    GitRefSyntax,
    #[error("invalid hostname syntax")]
    HostnameSyntax,
    #[error("invalid HTTPS origin syntax")]
    OriginSyntax,
    #[error("invalid currency syntax")]
    CurrencySyntax,
    #[error("invalid Git path syntax")]
    GitPathSyntax,
    #[error("option array contains more than one element")]
    OptionLength,
    #[error("unsupported wire revision")]
    WireRevision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SetError {
    #[error("set is unsorted in canonical encoded order")]
    Unsorted,
    #[error("duplicate canonical set element")]
    Duplicate,
    #[error("array exceeds maximum length 65536")]
    TooMany,
    #[error("duplicate record identity")]
    DuplicateIdentity,
}

/// Comparison of complete canonical encoded values, without exposing a wire codec.
#[doc(hidden)]
pub trait CanonicalOrder {
    fn canonical_cmp(&self, other: &Self) -> Ordering;
    fn canonical_size(&self) -> usize;
    fn canonical_nesting(&self) -> usize {
        0
    }
    fn identity_key(&self) -> Option<Vec<u8>> {
        None
    }
}

// ENCODING.md §Exact bytes fixes record size and nesting limits.
const MAX_RECORD_BYTES: usize = 16_777_216;
const MAX_NESTING: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ProjectionError {
    #[error("record exceeds maximum canonical size 16777216 bytes")]
    RecordTooLarge,
    #[error("record exceeds maximum nesting depth 32")]
    TooDeep,
}

#[doc(hidden)]
pub fn validate_projection_limits(size: usize, nesting: usize) -> Result<(), ProjectionError> {
    if size > MAX_RECORD_BYTES {
        return Err(ProjectionError::RecordTooLarge);
    }
    if nesting > MAX_NESTING {
        return Err(ProjectionError::TooDeep);
    }
    Ok(())
}

/// Size accounting for a shortest integer or length header; no bytes are encoded.
#[doc(hidden)]
pub fn canonical_header_len(value: u64) -> usize {
    match value {
        0..=23 => 1,
        24..=255 => 2,
        256..=65_535 => 3,
        65_536..=4_294_967_295 => 5,
        _ => 9,
    }
}

fn signed_header_len(value: i64) -> usize {
    let argument = if value >= 0 {
        value as u64
    } else {
        value.unsigned_abs() - 1
    };
    canonical_header_len(argument)
}

fn content_size(length: usize) -> usize {
    canonical_header_len(length as u64).saturating_add(length)
}

fn array_size<T: CanonicalOrder>(values: &[T]) -> usize {
    values
        .iter()
        .fold(canonical_header_len(values.len() as u64), |total, value| {
            total.saturating_add(value.canonical_size())
        })
}

fn array_nesting<T: CanonicalOrder>(values: &[T]) -> usize {
    values
        .iter()
        .map(CanonicalOrder::canonical_nesting)
        .max()
        .unwrap_or(0)
        .saturating_add(1)
}

fn unsigned_cmp(left: u64, right: u64) -> Ordering {
    // Shortest integer headers increase lexicographically with their argument.
    left.cmp(&right)
}

fn signed_cmp(left: i64, right: i64) -> Ordering {
    match (left >= 0, right >= 0) {
        (true, true) => unsigned_cmp(left as u64, right as u64),
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => unsigned_cmp(left.unsigned_abs() - 1, right.unsigned_abs() - 1),
    }
}

fn content_cmp(left: &[u8], right: &[u8]) -> Ordering {
    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}

impl CanonicalOrder for bool {
    fn canonical_size(&self) -> usize {
        1
    }
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}
macro_rules! unsigned_order {
    ($($t:ty),+) => { $(impl CanonicalOrder for $t {
        fn canonical_cmp(&self, other: &Self) -> Ordering { unsigned_cmp(u64::from(*self), u64::from(*other)) }
        fn canonical_size(&self) -> usize { canonical_header_len(u64::from(*self)) }
    })+ };
}
unsigned_order!(u16, u32);
impl CanonicalOrder for i16 {
    fn canonical_size(&self) -> usize {
        signed_header_len(i64::from(*self))
    }
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        signed_cmp(i64::from(*self), i64::from(*other))
    }
}
impl CanonicalOrder for String {
    fn canonical_size(&self) -> usize {
        content_size(self.len())
    }
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        content_cmp(self.as_bytes(), other.as_bytes())
    }
}
impl CanonicalOrder for str {
    fn canonical_size(&self) -> usize {
        content_size(self.len())
    }
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        content_cmp(self.as_bytes(), other.as_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U64Str(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I64Str(pub i64);

fn canonical_digits(value: &str) -> bool {
    value == "0"
        || (!value.starts_with('0')
            && !value.is_empty()
            && value.bytes().all(|b| b.is_ascii_digit()))
}
impl U64Str {
    pub fn parse(value: &str) -> Result<Self, PrimitiveError> {
        if !canonical_digits(value) {
            return Err(PrimitiveError::DecimalSyntax);
        }
        value
            .parse()
            .map(Self)
            .map_err(|_| PrimitiveError::DecimalOverflow)
    }
}
impl I64Str {
    pub fn parse(value: &str) -> Result<Self, PrimitiveError> {
        let digits = value.strip_prefix('-').unwrap_or(value);
        if !canonical_digits(digits) || value == "-0" {
            return Err(PrimitiveError::DecimalSyntax);
        }
        value
            .parse()
            .map(Self)
            .map_err(|_| PrimitiveError::DecimalOverflow)
    }
}
macro_rules! decimal_serde {
    ($t:ident, $cmp:ident, $size:ident) => {
        impl Serialize for $t {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str(&self.0)
            }
        }
        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                Self::parse(&value).map_err(de::Error::custom)
            }
        }
        impl CanonicalOrder for $t {
            fn canonical_cmp(&self, other: &Self) -> Ordering {
                $cmp(self.0, other.0)
            }
            fn canonical_size(&self) -> usize {
                $size(self.0)
            }
        }
    };
}
decimal_serde!(U64Str, unsigned_cmp, canonical_header_len);
decimal_serde!(I64Str, signed_cmp, signed_header_len);

fn decode_hex(value: &str, maximum: usize) -> Result<Vec<u8>, PrimitiveError> {
    if value.len() / 2 > maximum {
        return Err(PrimitiveError::ByteLimit);
    }
    if !value.len().is_multiple_of(2)
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(PrimitiveError::HexSyntax);
    }
    hex::decode(value).map_err(|_| PrimitiveError::HexSyntax)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hex<const N: usize>(pub [u8; N]);
impl<const N: usize> Serialize for Hex<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&hex::encode(self.0))
    }
}
impl<'de, const N: usize> Deserialize<'de> for Hex<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        if value.len()
            != N.checked_mul(2)
                .ok_or_else(|| de::Error::custom(PrimitiveError::HexLength))?
        {
            return Err(de::Error::custom(PrimitiveError::HexLength));
        }
        let bytes = decode_hex(&value, N).map_err(de::Error::custom)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| de::Error::custom(PrimitiveError::HexLength))?;
        Ok(Self(bytes))
    }
}
impl<const N: usize> CanonicalOrder for Hex<N> {
    fn canonical_size(&self) -> usize {
        content_size(N)
    }
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bytes(pub Vec<u8>);
impl Bytes {
    pub fn new(value: Vec<u8>) -> Result<Self, PrimitiveError> {
        let result = Self(value);
        result.validate()?;
        Ok(result)
    }
    pub fn validate(&self) -> Result<(), PrimitiveError> {
        if self.0.len() > MAX_BYTES_LEN {
            return Err(PrimitiveError::ByteLimit);
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitPath(pub Vec<u8>);
impl GitPath {
    pub fn new(value: Vec<u8>) -> Result<Self, PrimitiveError> {
        let result = Self(value);
        result.validate()?;
        Ok(result)
    }
    pub fn validate(&self) -> Result<(), PrimitiveError> {
        if self.0.len() > MAX_GIT_PATH_LEN {
            return Err(PrimitiveError::ByteLimit);
        }
        if self.0.is_empty()
            || self.0.contains(&0)
            || self.0.contains(&b'\\')
            || self
                .0
                .split(|b| *b == b'/')
                .any(|p| p.is_empty() || p == b"." || p == b"..")
        {
            return Err(PrimitiveError::GitPathSyntax);
        }
        Ok(())
    }
}
macro_rules! bytes_serde {
    ($t:ident, $max:ident) => {
        impl Serialize for $t {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.validate().map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&hex::encode(&self.0))
            }
        }
        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                let bytes = decode_hex(&value, $max).map_err(de::Error::custom)?;
                Self::new(bytes).map_err(de::Error::custom)
            }
        }
        impl CanonicalOrder for $t {
            fn canonical_cmp(&self, other: &Self) -> Ordering {
                content_cmp(&self.0, &other.0)
            }
            fn canonical_size(&self) -> usize {
                content_size(self.0.len())
            }
        }
    };
}
bytes_serde!(Bytes, MAX_BYTES_LEN);
bytes_serde!(GitPath, MAX_GIT_PATH_LEN);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uuid(pub [u8; 16]);
impl Serialize for Uuid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&uuid::Uuid::from_bytes(self.0).hyphenated())
    }
}
impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        if value.len() != 36
            || !value.bytes().enumerate().all(|(i, b)| {
                if [8, 13, 18, 23].contains(&i) {
                    b == b'-'
                } else {
                    b.is_ascii_digit() || (b'a'..=b'f').contains(&b)
                }
            })
        {
            return Err(de::Error::custom(PrimitiveError::UuidSyntax));
        }
        let parsed = uuid::Uuid::parse_str(&value)
            .map_err(|_| de::Error::custom(PrimitiveError::UuidSyntax))?;
        Ok(Self(*parsed.as_bytes()))
    }
}
impl CanonicalOrder for Uuid {
    fn canonical_size(&self) -> usize {
        content_size(self.0.len())
    }
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

fn validate_text(value: &str, min: usize, max: usize) -> Result<(), PrimitiveError> {
    if value.len() > max || value.chars().count() < min {
        return Err(PrimitiveError::TextLength);
    }
    if !value.nfc().eq(value.chars()) {
        return Err(PrimitiveError::NonNfc);
    }
    Ok(())
}
fn validate_identifier(value: &str) -> Result<(), PrimitiveError> {
    validate_text(value, 1, 256)?;
    if !value.as_bytes()[0].is_ascii_alphanumeric()
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:/@+ -".contains(&b))
    {
        return Err(PrimitiveError::IdentifierSyntax);
    }
    Ok(())
}
fn validate_git_ref(value: &str) -> Result<(), PrimitiveError> {
    validate_text(value, 12, 1024)?;
    if !value.starts_with("refs/heads/")
        || value
            .chars()
            .any(|c| c <= '\u{20}' || " ~^:?*\\[".contains(c))
        || value.contains("..")
        || value.contains("@{")
        || value.ends_with('.')
        || value
            .split('/')
            .any(|part| part.is_empty() || part.starts_with('.') || part.ends_with(".lock"))
    {
        return Err(PrimitiveError::GitRefSyntax);
    }
    Ok(())
}
fn hostname_syntax(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 253
        && value.split('.').all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        })
}
fn validate_hostname(value: &str) -> Result<(), PrimitiveError> {
    validate_text(value, 1, 253)?;
    if !value
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b".-".contains(&b))
    {
        return Err(PrimitiveError::HostnameSyntax);
    }
    Ok(())
}
fn validate_origin(value: &str) -> Result<(), PrimitiveError> {
    validate_text(value, 1, 512)?;
    let authority = value
        .strip_prefix("https://")
        .ok_or(PrimitiveError::OriginSyntax)?;
    let host = match authority.split_once(':') {
        None => authority,
        Some((host, port)) => {
            if port.is_empty() || !port.bytes().all(|b| b.is_ascii_digit()) {
                return Err(PrimitiveError::OriginSyntax);
            }
            let port: u16 = port.parse().map_err(|_| PrimitiveError::OriginSyntax)?;
            if port == 443 {
                return Err(PrimitiveError::OriginSyntax);
            }
            host
        }
    };
    if !hostname_syntax(host) {
        return Err(PrimitiveError::OriginSyntax);
    }
    Ok(())
}
fn validate_currency(value: &str) -> Result<(), PrimitiveError> {
    validate_text(value, 3, 3)?;
    if !value.bytes().all(|b| b.is_ascii_uppercase()) {
        return Err(PrimitiveError::CurrencySyntax);
    }
    Ok(())
}
macro_rules! text_type {
    ($t:ident, $validator:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $t(pub String);
        impl $t {
            pub fn new(value: String) -> Result<Self, PrimitiveError> {
                let result = Self(value);
                result.validate()?;
                Ok(result)
            }
            pub fn validate(&self) -> Result<(), PrimitiveError> {
                $validator(&self.0)
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl Serialize for $t {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.validate().map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&self.0)
            }
        }
        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                Self::new(String::deserialize(deserializer)?).map_err(de::Error::custom)
            }
        }
        impl CanonicalOrder for $t {
            fn canonical_cmp(&self, other: &Self) -> Ordering {
                content_cmp(self.0.as_bytes(), other.0.as_bytes())
            }
            fn canonical_size(&self) -> usize {
                content_size(self.0.len())
            }
        }
    };
}
text_type!(Identifier, validate_identifier);
text_type!(GitRef, validate_git_ref);
text_type!(Hostname, validate_hostname);
text_type!(Origin, validate_origin);
text_type!(Currency, validate_currency);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opt<T>(pub Option<T>);
impl<T: Serialize> Serialize for Opt<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(usize::from(self.0.is_some())))?;
        if let Some(value) = &self.0 {
            seq.serialize_element(value)?;
        }
        seq.end()
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Opt<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct OptionVisitor<T>(std::marker::PhantomData<T>);
        impl<'de, T: Deserialize<'de>> Visitor<'de> for OptionVisitor<T> {
            type Value = Opt<T>;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a zero or one element option array")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                if seq.size_hint().is_some_and(|len| len > 1) {
                    return Err(de::Error::custom(PrimitiveError::OptionLength));
                }
                let value = seq.next_element()?;
                if seq.next_element::<de::IgnoredAny>()?.is_some() {
                    return Err(de::Error::custom(PrimitiveError::OptionLength));
                }
                Ok(Opt(value))
            }
        }
        deserializer.deserialize_seq(OptionVisitor(std::marker::PhantomData))
    }
}
impl<T: CanonicalOrder> CanonicalOrder for Opt<T> {
    fn canonical_size(&self) -> usize {
        self.0
            .as_ref()
            .map_or(1, |value| 1usize.saturating_add(value.canonical_size()))
    }
    fn canonical_nesting(&self) -> usize {
        self.0
            .as_ref()
            .map_or(1, |value| value.canonical_nesting().saturating_add(1))
    }
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        match (&self.0, &other.0) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => a.canonical_cmp(b),
        }
    }
}
impl<T: CanonicalOrder> CanonicalOrder for Vec<T> {
    fn canonical_size(&self) -> usize {
        array_size(self)
    }
    fn canonical_nesting(&self) -> usize {
        array_nesting(self)
    }
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.len().cmp(&other.len()).then_with(|| {
            self.iter()
                .zip(other)
                .map(|(a, b)| a.canonical_cmp(b))
                .find(|o| *o != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
    }
}

fn validate_identities<T: CanonicalOrder>(values: &[T]) -> Result<(), SetError> {
    let mut identities = HashSet::new();
    for value in values {
        if let Some(identity) = value.identity_key()
            && !identities.insert(identity)
        {
            return Err(SetError::DuplicateIdentity);
        }
    }
    Ok(())
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetOf<T>(Vec<T>);
impl<T: CanonicalOrder> SetOf<T> {
    pub fn new(values: Vec<T>) -> Result<Self, SetError> {
        if values.len() > MAX_ARRAY_LEN {
            return Err(SetError::TooMany);
        }
        for pair in values.windows(2) {
            match pair[0].canonical_cmp(&pair[1]) {
                Ordering::Greater => return Err(SetError::Unsorted),
                Ordering::Equal => return Err(SetError::Duplicate),
                Ordering::Less => {}
            }
        }
        validate_identities(&values)?;
        Ok(Self(values))
    }
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}
impl<T: Serialize> Serialize for SetOf<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}
impl<'de, T: Deserialize<'de> + CanonicalOrder> Deserialize<'de> for SetOf<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::new(bounded_array(deserializer)?).map_err(de::Error::custom)
    }
}
impl<T: CanonicalOrder> CanonicalOrder for SetOf<T> {
    fn canonical_size(&self) -> usize {
        array_size(&self.0)
    }
    fn canonical_nesting(&self) -> usize {
        array_nesting(&self.0)
    }
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.0.canonical_cmp(&other.0)
    }
}

/// Bounded ordered arrays retain order and reject repeated named record identities.
#[doc(hidden)]
pub fn bounded_array<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + CanonicalOrder,
{
    struct ArrayVisitor<T>(std::marker::PhantomData<T>);
    impl<'de, T: Deserialize<'de> + CanonicalOrder> Visitor<'de> for ArrayVisitor<T> {
        type Value = Vec<T>;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an array of at most 65536 elements")
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            if seq.size_hint().is_some_and(|len| len > MAX_ARRAY_LEN) {
                return Err(de::Error::custom(SetError::TooMany));
            }
            let mut values = Vec::new();
            while values.len() < MAX_ARRAY_LEN {
                match seq.next_element()? {
                    Some(value) => values.push(value),
                    None => {
                        validate_identities(&values).map_err(de::Error::custom)?;
                        return Ok(values);
                    }
                }
            }
            // Detect a surplus element without allocating another decoded value.
            if seq.next_element::<de::IgnoredAny>()?.is_some() {
                return Err(de::Error::custom(SetError::TooMany));
            }
            validate_identities(&values).map_err(de::Error::custom)?;
            Ok(values)
        }
    }
    deserializer.deserialize_seq(ArrayVisitor(std::marker::PhantomData))
}

/// Serialization validates public Vec fields just as deserialization does.
#[doc(hidden)]
pub fn serialize_bounded_array<S, T>(values: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + CanonicalOrder,
{
    if values.len() > MAX_ARRAY_LEN {
        return Err(serde::ser::Error::custom(SetError::TooMany));
    }
    validate_identities(values).map_err(serde::ser::Error::custom)?;
    values.serialize(serializer)
}

/// ENCODING.md fixes the only supported wire revision to the JSON integer 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WireRevision;
impl Serialize for WireRevision {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u16(1)
    }
}
impl<'de> Deserialize<'de> for WireRevision {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if u16::deserialize(deserializer)? != 1 {
            return Err(de::Error::custom(PrimitiveError::WireRevision));
        }
        Ok(Self)
    }
}
impl CanonicalOrder for WireRevision {
    fn canonical_size(&self) -> usize {
        1
    }
    fn canonical_cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

/// Borrowed serialization adapter for generated bounded array fields.
#[doc(hidden)]
pub struct BoundedArrayRef<'a, T>(pub &'a [T]);
impl<T: Serialize + CanonicalOrder> Serialize for BoundedArrayRef<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_bounded_array(self.0, serializer)
    }
}

/// Require JSON objects while retaining serde's duplicate and unknown-field checks.
#[doc(hidden)]
pub struct ObjectOnly<T>(pub T);
impl<'de, T: Deserialize<'de>> Deserialize<'de> for ObjectOnly<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ObjectVisitor<T>(std::marker::PhantomData<T>);
        impl<'de, T: Deserialize<'de>> Visitor<'de> for ObjectVisitor<T> {
            type Value = ObjectOnly<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a contract JSON object")
            }

            fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                T::deserialize(de::value::MapAccessDeserializer::new(map)).map(ObjectOnly)
            }
        }
        deserializer.deserialize_map(ObjectVisitor(std::marker::PhantomData))
    }
}
