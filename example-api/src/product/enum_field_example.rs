use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumFieldExample {
    enum_: super::EnumExample,
}
impl EnumFieldExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(enum_: super::EnumExample) -> EnumFieldExample {
        EnumFieldExample { enum_: enum_ }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn enum_(&self) -> &super::EnumExample {
        &self.enum_
    }
}
///A builder for the `EnumFieldExample` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    enum_: Option<super::EnumExample>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn enum_(&mut self, enum_: super::EnumExample) -> &mut Self {
        self.enum_ = Some(enum_);
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> EnumFieldExample {
        EnumFieldExample {
            enum_: self.enum_.clone().expect("field enum_ was not set"),
        }
    }
}
impl From<EnumFieldExample> for Builder {
    #[inline]
    fn from(_v: EnumFieldExample) -> Builder {
        Builder { enum_: Some(_v.enum_) }
    }
}
impl ser::Serialize for EnumFieldExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("EnumFieldExample", size)?;
        s.serialize_field("enum", &self.enum_)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for EnumFieldExample {
    fn deserialize<D>(d: D) -> Result<EnumFieldExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("EnumFieldExample", &["enum"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = EnumFieldExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<EnumFieldExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut enum_ = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Enum => enum_ = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let enum_ = match enum_ {
            Some(v) => v,
            None => return Err(de::Error::missing_field("enum")),
        };
        Ok(EnumFieldExample { enum_ })
    }
}
enum Field_ {
    Enum,
    Unknown_,
}
impl<'de> de::Deserialize<'de> for Field_ {
    fn deserialize<D>(d: D) -> Result<Field_, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(FieldVisitor_)
    }
}
struct FieldVisitor_;
impl<'de> de::Visitor<'de> for FieldVisitor_ {
    type Value = Field_;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E>(self, value: &str) -> Result<Field_, E>
    where
        E: de::Error,
    {
        let v = match value {
            "enum" => Field_::Enum,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
