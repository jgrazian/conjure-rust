use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct QueryParameterType {
    param_id: super::ParameterId,
}
impl QueryParameterType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(param_id: super::ParameterId) -> QueryParameterType {
        QueryParameterType {
            param_id: param_id,
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn param_id(&self) -> &super::ParameterId {
        &self.param_id
    }
}
///A builder for the `QueryParameterType` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    param_id: Option<super::ParameterId>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn param_id(&mut self, param_id: super::ParameterId) -> &mut Self {
        self.param_id = Some(param_id);
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> QueryParameterType {
        QueryParameterType {
            param_id: self.param_id.clone().expect("field param_id was not set"),
        }
    }
}
impl From<QueryParameterType> for Builder {
    #[inline]
    fn from(_v: QueryParameterType) -> Builder {
        Builder {
            param_id: Some(_v.param_id),
        }
    }
}
impl ser::Serialize for QueryParameterType {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("QueryParameterType", size)?;
        s.serialize_field("paramId", &self.param_id)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for QueryParameterType {
    fn deserialize<D>(d: D) -> Result<QueryParameterType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("QueryParameterType", &["paramId"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = QueryParameterType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<QueryParameterType, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut param_id = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::ParamId => param_id = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let param_id = match param_id {
            Some(v) => v,
            None => return Err(de::Error::missing_field("paramId")),
        };
        Ok(QueryParameterType { param_id })
    }
}
enum Field_ {
    ParamId,
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
            "paramId" => Field_::ParamId,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
