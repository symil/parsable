#[macro_export]
macro_rules! create_token_struct {
    ($struct_name:ident, $content:expr) => {
        #[derive(Debug)]
        pub struct $struct_name {
            pub token: &'static str,
            pub location: parsable::ItemLocation
        }

        impl parsable::Parsable for $struct_name {
            fn parse_item(reader: &mut parsable::StringReader) -> Option<Self> {
                let start = reader.get_index();

                match reader.read_string($content) {
                    Some(_) => Some(Self {
                        token: $content,
                        location: reader.get_item_location(start),
                    }),
                    None => None,
                }
            }

            fn get_item_name() -> String {
                format!("\"{}\"", $content)
            }

            fn location(&self) -> &parsable::ItemLocation {
                &self.location
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = parsable::ItemLocation;

            fn deref(&self) -> &Self::Target {
                &self.location
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    token: $content,
                    location: Default::default()
                }
            }
        }
    }
}