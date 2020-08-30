use core::slice::Iter;
use std::collections::HashMap;

/// A JSON value.
#[derive(Clone, PartialEq, Debug)]
pub enum JsonValue {
    String(String),
    Number(String),
    Boolean(bool),
    Object(JsonObject),
    Array(JsonArray),
    Null,
}

/// A JSON object.
#[derive(Clone, PartialEq, Debug)]
pub struct JsonObject(HashMap<String, JsonValue>);

impl IntoIterator for JsonObject {
    type Item = (String, JsonValue);
    type IntoIter = std::collections::hash_map::IntoIter<String, JsonValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<HashMap<String, JsonValue>> for JsonObject {
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

impl JsonObject {
    /// Creates a new JsonObject.
    pub fn new(inner: HashMap<String, JsonValue>) -> JsonObject {
        JsonObject(inner)
    }

    /// Drops the object returning the inner hash map.
    pub fn take_inner(self) -> HashMap<String, JsonValue> {
        self.0
    }

    /// Gets the number of properties.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets a value in the object by its name.
    pub fn get(&self, name: &str) -> Option<&JsonValue> {
        self.0.get(name)
    }

    /// Gets a string property value from the object by name.
    /// Returns `None` when not a string or it doesn't exist.
    pub fn get_string(&self, name: &str) -> Option<&String> {
        generate_get!(self, name, String)
    }

    /// Gets a number property value from the object by name.
    /// Returns `None` when not a number or it doesn't exist.
    pub fn get_number(&self, name: &str) -> Option<&String> {
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
    pub fn get_object(&self, name: &str) -> Option<&JsonObject> {
        generate_get!(self, name, Object)
    }

    /// Gets an array property value from the object by name.
    /// Returns `None` when not an array or it doesn't exist.
    pub fn get_array(&self, name: &str) -> Option<&JsonArray> {
        generate_get!(self, name, Array)
    }

    /// Takes a value from the object by name.
    /// Returns `None` when it doesn't exist.
    pub fn take(&mut self, name: &str) -> Option<JsonValue> {
        self.0.remove(name)
    }

    /// Takes a string property value from the object by name.
    /// Returns `None` when not a string or it doesn't exist.
    pub fn take_string(&mut self, name: &str) -> Option<String> {
        generate_take!(self, name, String)
    }

    /// Takes a number property value from the object by name.
    /// Returns `None` when not a number or it doesn't exist.
    pub fn take_number(&mut self, name: &str) -> Option<String> {
        generate_take!(self, name, Number)
    }

    /// Takes a boolean property value from the object by name.
    /// Returns `None` when not a boolean or it doesn't exist.
    pub fn take_boolean(&mut self, name: &str) -> Option<bool> {
        generate_take!(self, name, Boolean)
    }

    /// Takes an object property value from the object by name.
    /// Returns `None` when not an object or it doesn't exist.
    pub fn take_object(&mut self, name: &str) -> Option<JsonObject> {
        generate_take!(self, name, Object)
    }

    /// Takes an array property value from the object by name.
    /// Returns `None` when not an array or it doesn't exist.
    pub fn take_array(&mut self, name: &str) -> Option<JsonArray> {
        generate_take!(self, name, Array)
    }
}

/// A JSON array.
#[derive(Clone, PartialEq, Debug)]
pub struct JsonArray(Vec<JsonValue>);

impl IntoIterator for JsonArray {
    type Item = JsonValue;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Vec<JsonValue>> for JsonArray {
    fn from(elements: Vec<JsonValue>) -> JsonArray {
        JsonArray::new(elements)
    }
}

impl JsonArray {
    /// Creates a new JsonArray.
    pub fn new(inner: Vec<JsonValue>) -> JsonArray {
        JsonArray(inner)
    }

    /// Drops the object returning the inner vector.
    pub fn take_inner(self) -> Vec<JsonValue> {
        self.0
    }

    /// Iterates over the array elements.
    pub fn iter(&self) -> Iter<JsonValue> {
        self.0.iter()
    }

    /// Gets a value from the array by index.
    pub fn get(&self, index: usize) -> Option<&JsonValue> {
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
        inner.insert(String::from("prop"), JsonValue::String(String::from("asdf")));
        inner.insert(String::from("other"), JsonValue::String(String::from("text")));
        let mut obj = JsonObject::new(inner);

        assert_eq!(obj.len(), 2);
        assert_eq!(obj.take_string("asdf"), None);
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.take_number("prop"), None);
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.take_string("prop"), Some(String::from("asdf")));
        assert_eq!(obj.len(), 1);
        assert_eq!(obj.take("something"), None);
        assert_eq!(obj.len(), 1);
        assert_eq!(obj.take("other"), Some(JsonValue::String(String::from("text"))));
        assert_eq!(obj.len(), 0);
    }

    #[test]
    fn it_should_get() {
        let mut inner = HashMap::new();
        inner.insert(String::from("prop"), JsonValue::String(String::from("asdf")));
        let obj = JsonObject::new(inner);

        assert_eq!(obj.len(), 1);
        assert_eq!(obj.get_string("asdf"), None);
        assert_eq!(obj.get_string("prop"), Some(&String::from("asdf")));
        assert_eq!(obj.get("prop"), Some(&JsonValue::String(String::from("asdf"))));
        assert_eq!(obj.get("asdf"), None);
        assert_eq!(obj.len(), 1);
    }
}
