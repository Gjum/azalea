use std::fmt::{self, Formatter};

use crate::{
    base_component::BaseComponent, component::Component, style::Style,
    text_component::TextComponent,
};

#[derive(Clone, Debug)]
pub enum StringOrComponent {
    String(String),
    Component(Component),
}

#[derive(Clone, Debug)]
pub struct TranslatableComponent {
    pub base: BaseComponent,
    pub key: String,
    pub args: Vec<StringOrComponent>,
}

impl TranslatableComponent {
    pub fn new(key: String, args: Vec<StringOrComponent>) -> Self {
        Self {
            base: BaseComponent::new(),
            key,
            args,
        }
    }

    /// Convert the key and args to a Component.
    pub fn read(&self) -> Result<TextComponent, fmt::Error> {
        let template = azalea_language::get(&self.key).unwrap_or(&self.key);
        // decode the % things

        let mut i = 0;
        let mut matched = 0;

        // every time we get a char we add it to built_text, and we push it to
        // `arguments` and clear it when we add a new argument component
        let mut built_text = String::new();
        let mut components = Vec::new();

        while i < template.len() {
            if template.chars().nth(i).unwrap() == '%' {
                let char_after = match template.chars().nth(i + 1) {
                    Some(c) => c,
                    None => {
                        built_text.push(template.chars().nth(i).unwrap());
                        break;
                    }
                };
                i += 1;
                match char_after {
                    '%' => {
                        built_text.push('%');
                    }
                    's' => {
                        let arg_component = self
                            .args
                            .get(matched)
                            .cloned()
                            .unwrap_or(StringOrComponent::String("".to_string()));

                        components.push(TextComponent::new(built_text.clone()));
                        built_text.clear();
                        components.push(TextComponent::from(arg_component));
                        matched += 1;
                    }
                    _ => {
                        // check if the char is a number
                        if let Some(d) = char_after.to_digit(10) {
                            // make sure the next two chars are $s
                            if let Some('$') = template.chars().nth(i + 1) {
                                if let Some('s') = template.chars().nth(i + 2) {
                                    i += 2;
                                    built_text.push_str(
                                        &self
                                            .args
                                            .get((d - 1) as usize)
                                            .unwrap_or(&StringOrComponent::String("".to_string()))
                                            .to_string(),
                                    );
                                } else {
                                    return Err(fmt::Error);
                                }
                            } else {
                                return Err(fmt::Error);
                            }
                        } else {
                            i -= 1;
                            built_text.push('%');
                        }
                    }
                }
            } else {
                built_text.push(template.chars().nth(i).unwrap());
            }

            i += 1
        }

        if components.is_empty() {
            return Ok(TextComponent::new(built_text));
        }

        components.push(TextComponent::new(built_text));

        Ok(TextComponent {
            base: BaseComponent {
                siblings: components.into_iter().map(|c| Component::Text(c)).collect(),
                style: Style::default(),
            },
            text: "".to_string(),
        })
    }
}

impl fmt::Display for TranslatableComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Component::Translatable(self.clone()))
    }
}

impl fmt::Display for StringOrComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            StringOrComponent::String(s) => write!(f, "{}", s),
            StringOrComponent::Component(c) => write!(f, "{}", c.to_string()),
        }
    }
}

impl From<StringOrComponent> for TextComponent {
    fn from(soc: StringOrComponent) -> Self {
        match soc {
            StringOrComponent::String(s) => TextComponent::new(s),
            StringOrComponent::Component(c) => TextComponent::new(c.to_string()),
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let c = TranslatableComponent::new("translation.test.none".to_string(), vec![]);
        assert_eq!(c.read().unwrap().to_string(), "Hello, world!".to_string());
    }
    #[test]
    fn test_complex() {
        let c = TranslatableComponent::new(
            "translation.test.complex".to_string(),
            vec![
                StringOrComponent::String("a".to_string()),
                StringOrComponent::String("b".to_string()),
                StringOrComponent::String("c".to_string()),
                StringOrComponent::String("d".to_string()),
            ],
        );
        // so true mojang
        assert_eq!(
            c.read().unwrap().to_string(),
            "Prefix, ab again b and a lastly c and also a again!".to_string()
        );
    }
    #[test]
    fn test_escape() {
        let c = TranslatableComponent::new(
            "translation.test.escape".to_string(),
            vec![
                StringOrComponent::String("a".to_string()),
                StringOrComponent::String("b".to_string()),
                StringOrComponent::String("c".to_string()),
                StringOrComponent::String("d".to_string()),
            ],
        );
        assert_eq!(c.read().unwrap().to_string(), "%s %a %%s %%b".to_string());
    }
    #[test]
    fn test_invalid() {
        let c = TranslatableComponent::new(
            "translation.test.invalid".to_string(),
            vec![
                StringOrComponent::String("a".to_string()),
                StringOrComponent::String("b".to_string()),
                StringOrComponent::String("c".to_string()),
                StringOrComponent::String("d".to_string()),
            ],
        );
        assert_eq!(c.read().unwrap().to_string(), "hi %".to_string());
    }
    #[test]
    fn test_invalid2() {
        let c = TranslatableComponent::new(
            "translation.test.invalid2".to_string(),
            vec![
                StringOrComponent::String("a".to_string()),
                StringOrComponent::String("b".to_string()),
                StringOrComponent::String("c".to_string()),
                StringOrComponent::String("d".to_string()),
            ],
        );
        assert_eq!(c.read().unwrap().to_string(), "hi %  s".to_string());
    }
}
