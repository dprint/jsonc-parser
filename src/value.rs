use core::slice::Iter;
// use std::borrow::Cow;
use beef::lean::Cow;
use std::collections::HashMap;

/// A JSON value.
#[derive(Clone, PartialEq, Debug)]
pub enum JsonValue<'a> {
    String(Cow<'a, str>),
    Number(&'a str),
    Boolean(bool),
    Object(JsonObject<'a>),
    Array(JsonArray<'a>),
    Null,
}

/// A JSON object.
#[derive(Clone, PartialEq, Debug)]
pub struct JsonObject<'a>(HashMap<String, JsonValue<'a>>);

impl<'a> IntoIterator for JsonObject<'a> {
    type Item = (String, JsonValue<'a>);
    type IntoIter = std::collections::hash_map::IntoIter<String, JsonValue<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> From<HashMap<String, JsonValue<'a>>> for JsonObject<'a> {
    fn from(properties: HashMap<String, JsonValue>) -> JsonObject {
        JsonObject::new(properties)
    }
}

macro_rules! generate_take {
    ($self:ident, $name:ident, $value_type:ident) => {
        match $self.0.remove_entry($name) {
            Some((_, JsonValue::$value_type(value))) => Some(value),
            Some((key, value)) => {
                // add it back
                $self.0.insert(key, value);
                None
            }
            _ => None,
        }
    };
}

macro_rules! generate_get {
    ($self:ident, $name:ident, $value_type:ident) => {
        match $self.0.get($name) {
            Some(JsonValue::$value_type(value)) => Some(value),
            _ => None,
        }
    };
}

impl<'a> JsonObject<'a> {
    /// Creates a new JsonObject.
    pub fn new(inner: HashMap<String, JsonValue<'a>>) -> JsonObject<'a> {
        JsonObject(inner)
    }

    /// Drops the object returning the inner hash map.
    pub fn take_inner(self) -> HashMap<String, JsonValue<'a>> {
        self.0
    }

    /// Gets the number of properties.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets a value in the object by its name.
    pub fn get(&self, name: &str) -> Option<&JsonValue<'a>> {
        self.0.get(name)
    }

    /// Gets a string property value from the object by name.
    /// Returns `None` when not a string or it doesn't exist.
    pub fn get_string(&self, name: &str) -> Option<&Cow<'a, str>> {
        generate_get!(self, name, String)
    }

    /// Gets a number property value from the object by name.
    /// Returns `None` when not a number or it doesn't exist.
    pub fn get_number(&self, name: &str) -> Option<&'a str> {
        generate_get!(self, name, Number)
    }

    /// Gets a boolean property value from the object by name.
    /// Returns `None` when not a boolean or it doesn't exist.
    pub fn get_boolean(&self, name: &str) -> Option<bool> {
        let result = generate_get!(self, name, Boolean);
        result.cloned()
    }

    /// Gets an object property value from the object by name.
    /// Returns `None` when not an object or it doesn't exist.
    pub fn get_object(&self, name: &str) -> Option<&JsonObject<'a>> {
        generate_get!(self, name, Object)
    }

    /// Gets an array property value from the object by name.
    /// Returns `None` when not an array or it doesn't exist.
    pub fn get_array(&self, name: &str) -> Option<&JsonArray<'a>> {
        generate_get!(self, name, Array)
    }

    /// Takes a value from the object by name.
    /// Returns `None` when it doesn't exist.
    pub fn take(&mut self, name: &str) -> Option<JsonValue<'a>> {
        self.0.remove(name)
    }

    /// Takes a string property value from the object by name.
    /// Returns `None` when not a string or it doesn't exist.
    pub fn take_string(&mut self, name: &str) -> Option<Cow<'a, str>> {
        generate_take!(self, name, String)
    }

    /// Takes a number property value from the object by name.
    /// Returns `None` when not a number or it doesn't exist.
    pub fn take_number(&mut self, name: &str) -> Option<&'a str> {
        generate_take!(self, name, Number)
    }

    /// Takes a boolean property value from the object by name.
    /// Returns `None` when not a boolean or it doesn't exist.
    pub fn take_boolean(&mut self, name: &str) -> Option<bool> {
        generate_take!(self, name, Boolean)
    }

    /// Takes an object property value from the object by name.
    /// Returns `None` when not an object or it doesn't exist.
    pub fn take_object(&mut self, name: &str) -> Option<JsonObject<'a>> {
        generate_take!(self, name, Object)
    }

    /// Takes an array property value from the object by name.
    /// Returns `None` when not an array or it doesn't exist.
    pub fn take_array(&mut self, name: &str) -> Option<JsonArray<'a>> {
        generate_take!(self, name, Array)
    }
}

/// A JSON array.
#[derive(Clone, PartialEq, Debug)]
pub struct JsonArray<'a>(Vec<JsonValue<'a>>);

impl<'a> IntoIterator for JsonArray<'a> {
    type Item = JsonValue<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> From<Vec<JsonValue<'a>>> for JsonArray<'a> {
    fn from(elements: Vec<JsonValue<'a>>) -> JsonArray<'a> {
        JsonArray::new(elements)
    }
}

impl<'a> JsonArray<'a> {
    /// Creates a new JsonArray.
    pub fn new(inner: Vec<JsonValue<'a>>) -> JsonArray<'a> {
        JsonArray(inner)
    }

    /// Drops the object returning the inner vector.
    pub fn take_inner(self) -> Vec<JsonValue<'a>> {
        self.0
    }

    /// Iterates over the array elements.
    pub fn iter(&self) -> Iter<JsonValue<'a>> {
        self.0.iter()
    }

    /// Gets a value from the array by index.
    pub fn get(&self, index: usize) -> Option<&JsonValue<'a>> {
        self.0.get(index)
    }

    /// Gets the number of elements.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn it_should_take() {
        let mut inner = HashMap::new();
        inner.insert(
            String::from("prop"),
            JsonValue::String(Cow::borrowed("asdf")),
        );
        inner.insert(
            String::from("other"),
            JsonValue::String(Cow::borrowed("text")),
        );
        let mut obj = JsonObject::new(inner);

        assert_eq!(obj.len(), 2);
        assert_eq!(obj.take_string("asdf"), None);
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.take_number("prop"), None);
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.take_string("prop"), Some(Cow::borrowed("asdf")));
        assert_eq!(obj.len(), 1);
        assert_eq!(obj.take("something"), None);
        assert_eq!(obj.len(), 1);
        assert_eq!(
            obj.take("other"),
            Some(JsonValue::String(Cow::borrowed("text")))
        );
        assert_eq!(obj.len(), 0);
    }

    #[test]
    fn it_should_get() {
        let mut inner = HashMap::new();
        inner.insert(
            String::from("prop"),
            JsonValue::String(Cow::borrowed("asdf")),
        );
        let obj = JsonObject::new(inner);

        assert_eq!(obj.len(), 1);
        assert_eq!(obj.get_string("asdf"), None);
        assert_eq!(obj.get_string("prop"), Some(&Cow::borrowed("asdf")));
        assert_eq!(
            obj.get("prop"),
            Some(&JsonValue::String(Cow::borrowed("asdf")))
        );
        assert_eq!(obj.get("asdf"), None);
        assert_eq!(obj.len(), 1);
    }
}
