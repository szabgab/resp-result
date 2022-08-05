use std::{fmt::Debug, ops::Add};

use http::{header::HeaderName, HeaderValue, StatusCode};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum ExtraFlag {
    EmptyBody,
    SetStatus(StatusCode),
    SetHeader(HeaderName, HeaderValue, HeaderType),
}
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum HeaderType {
    Insert,
    Append,
}

impl ExtraFlag {
    pub const fn empty_body() -> Self {
        Self::EmptyBody
    }

    pub const fn status(status: StatusCode) -> Self {
        Self::SetStatus(status)
    }

    pub fn append_header<K, V>(key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        K::Error: Debug,
        V: TryInto<HeaderValue>,
        V::Error: Debug,
    {
        Self::SetHeader(
            key.try_into().expect("Bad Header Name"),
            value.try_into().expect("Bad Header Value"),
            HeaderType::Append,
        )
    }
    pub fn insert_header<K, V>(key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        K::Error: Debug,
        V: TryInto<HeaderValue>,
        V::Error: Debug,
    {
        Self::SetHeader(
            key.try_into().expect("Bad Header Name"),
            value.try_into().expect("Bad Header Value"),
            HeaderType::Insert,
        )
    }
}

pub struct ExtraFlags {
    pub(crate) flags: Vec<ExtraFlag>,
}

impl From<()> for ExtraFlags {
    fn from(_: ()) -> Self {
        ExtraFlags { flags: vec![] }
    }
}

impl From<ExtraFlag> for ExtraFlags {
    fn from(flag: ExtraFlag) -> Self {
        Self {
            flags: Vec::from_iter([flag]),
        }
    }
}

impl Add for ExtraFlag {
    type Output = ExtraFlags;

    fn add(self, rhs: Self) -> Self::Output {
        ExtraFlags {
            flags: Vec::from([self, rhs]),
        }
    }
}

impl Add<ExtraFlags> for ExtraFlag {
    type Output = ExtraFlags;

    fn add(self, mut rhs: ExtraFlags) -> Self::Output {
        rhs.flags.push(self);
        rhs
    }
}

impl Add<ExtraFlag> for ExtraFlags {
    type Output = ExtraFlags;

    fn add(mut self, rhs: ExtraFlag) -> Self::Output {
        self.flags.push(rhs);
        self
    }
}
