// Crate inclusion.
use std::collections::HashMap;

// QueryString structure definition.
#[derive(Debug)]
pub struct QueryString<'buf>
{
    data: HashMap<&'buf str, Value<'buf>>,
}

// Value enumeraion definition.
#[derive(Debug)]
pub enum Value<'buf>
{
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

// QueryString structure implementation block.
impl<'buf> QueryString<'buf>
{
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// Implementation of From trait.
impl<'buf> From<&'buf str> for QueryString<'buf>
{
    fn from(s: &'buf str) -> Self
    {
        let mut data = HashMap::new();

        for sub_str in s.split('&')
        {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=')
            {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing
                {
                    Value::Single(prev_val) =>
                    {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
