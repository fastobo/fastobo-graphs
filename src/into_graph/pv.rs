use fastobo::ast::PropertyValue;
use crate::model::BasicPropertyValue;

impl From<PropertyValue> for BasicPropertyValue {
    fn from(pv: PropertyValue) -> Self {
        match pv {
            PropertyValue::Resource(rel, id) => {
                BasicPropertyValue::new(rel.to_string(), id.to_string())
            }
            PropertyValue::Literal(rel, value, ty) => {
                BasicPropertyValue::new(rel.to_string(), value.into_string())
            }
        }
    }
}
