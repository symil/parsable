## Installation

```
[dependencies]
parsable = "0.1"
```

## Example

Implementation of a basic operation interpreter that only works with positive integer and without operator priorities.

```rust
use parsable::{parsable, Parsable, ParseOptions};

#[parsable]
struct Operation {
    first_operand: Operand,
    other_operands: Vec<(Operator, Operand)>
}

impl Operation {
    fn process(&self) -> i32 {
        let mut result = self.first_operand.process();

        for (operator, operand) in &self.other_operands {
            let value = operand.process();

            result = match operator {
                Operator::Plus => result + value,
                Operator::Minus => result - value,
                Operator::Mult => result * value,
                Operator::Div => result / value,
                Operator::Mod => result % value,
            }
        }

        result
    }
}

#[parsable]
enum Operand {
    Number(NumberLiteral),
    Wrapped(WrappedOperation)
}

impl Operand {
    fn process(&self) -> i32 {
        match self {
            Operand::Number(number) => number.process(),
            Operand::Wrapped(wrapped) => wrapped.process(),
        }
    }
}

#[parsable]
struct NumberLiteral {
    #[parsable(regex=r"\d+")]
    value: String
}

impl NumberLiteral {
    fn process(&self) -> i32 {
        self.value.parse().unwrap()
    }
}

#[parsable]
struct WrappedOperation {
    #[parsable(brackets="()")]
    operation: Box<Operation>
}

impl WrappedOperation {
    fn process(&self) -> i32 {
        self.operation.process()
    }
}

#[parsable]
enum Operator {
    Plus = "+",
    Minus = "-",
    Mult = "*",
    Div = "/",
    Mod = "%"
}

fn main() {
    let operation_string = "3 + (4 * 5)".to_string();
    let parse_options = ParseOptions::default();
    
    match Operation::parse(operation_string, parse_options) {
        Ok(operation) => {
            println!("result: {}", operation.process());
        },
        Err(error) => {
            dbg!(error);
        }
    }
}
```

## The `#[parsable]` macro

Tagging a struct or enum with the `#[parsable]` macro implements the `Parsable` trait for the item, with the condition that all fields must also implement the `Parsable` trait. It can also be used on a field to tweak the way it is parsed.

### Struct

- All fields are parsed one after the other. The parsing is only successful if all fields are succesfully parsed.

### Enum

- The parsing stops on the first variant that is successfully parsed.
- If a variant contains multiple fields, they are parsed successively and must all be successful for the variant to be matched.
- If a variant contains no field, a string must be specified to indicate how to parse it.

```rust
#[parsable]
enum MyOperation {
    BinaryOperation(NumerLiteral, Operator, NumerLiteral),
    Number(NumberLiteral),
    Zero = "zero"
}

// If the first two variants are swapped, then the parsing will never reach the `SimpleOperation` variant
```

## Builtin types

### `String`

A string field must be tagged with the `#[parsable(regex="<pattern>")]` or `#[parsable(value="<pattern>")]` macro option to specify how to parse it.

```rust
// Matches at least one digit
#[parsable]
struct NumberLiteral {
    #[parsable(regex=r"\d+")]
    value: String
}
```

```rust

#[parsable]
// Only matches the string "+"
struct PlusSign {
    #[parsable(value="+")]
    value: String
}
```

### `Option<T>`

Matches `T`. If it fails, returns `None` but the parsing of the field is still considered successful.

```rust
#[parsable]
enum Sign {
    Plus = "+",
    Minus = "-"
}

// Matches a number with an optional sign
#[parsable]
struct NumberLiteral {
    sign: Option<Sign>,
    #[parsable(regex=r"\d+")]
    value: String
}
```

### `Vec<T>`

Matches as many `T` as possible successively. The following options can be specified:

- `min=X`: the parsing is only valid if at least X items are parsed
- `separator=<string>`: after each item, the parser will attempt to consume the separator. The parsing fails if no separator is found.

```rust
// Matches a non-empty list of numbers separated by a comma
#[parsable]
struct NumberList {
    #[parsable(separator=",", min=1)]
    numbers: Vec<NumberLiteral>
}
```

### Other types

- `()`: matches nothing, is always successful.
- `(T, U)`: matches `T`, then `U`.
- `Box<T>`: matches `T`.

## Running the parser

The `Parsable` trait provides the `parse()` method that takes two arguments:
- `content: String`: the string to parse
- `options: ParseOptions`: parse options

The `ParseOptions` type has the following fields:

- `comment_start: Option<&'static str>`: when the specified pattern is matched, the rest of the line is ignored. Common instances are `"//"` or `"#"`.
- `file_path: Option<String>`: file path of the string being parsed.
- `package_root_path: Option<String>`: root path of package or module containing the file being parsed.

The `file_path` and `package_root_path` fields are forwarded to the `FileInfo` struct and are never actually used by the library.

Blank characters (spaces, new lines and tabulations) are always ignored during parsing.

## File info

The `FileInfo` structure is used accross the library. It has the following fields:

- `content: String`: the string being parsed
- `path: String`: the path of the file being parsed, as specified in `ParseOptions`
- `package_root_path: String`: the path of the package containing the file, as specified in `ParseOptions`

It also provides the following methods:

- `get_line_col(index: usize) -> Option<(usize, usize)>`: returns the line and column numbers (starting at 1) associated with the specified character index. This method assumes 1 character per byte and therefore does not work properly when the file contains non-ascii characters.

## Item location

Tagging a struct with `#[parsable]` adds a `location` field of type `ItemLocation` with the following fields:

- `file: Rc<FileInfo>`: information on the file containing the item
- `start: usize`: starting index of the item in the file
- `end: usize`: ending index of the item in the file

The `Parsable` also trait provides a `location()` method:

- on a structure, it returns its `location` field
- on an enum, it returns the `location()` method of the variant that was matched
- calling `location()` on a variant with no field panics

A way to prevent the panic is to wrap enums with unit variants in a structure:

```rust
#[parsable]
enum Operator {
    Plus = "+",
    Minus = "-",
    Mult = "*",
    Div = "/",
    Mod = "%"
}

#[parsable]
struct WrappedOperator {
    operator: Operator
}

fn main() {
    let string = "+".to_string();
    let options = ParseOptions::default();
    let result = WrappedOperator::parse(string, options).unwrap();

    dbg!(result.location()); // It works!
}
```

## Parse error

On failure, `Parsable::parse()` returns `Err(ParseError)`. This structure has the following fields:

- `file: Rc<FileInfo>`: the file where the error occured.
- `index: usize`: the index at which the error occured.
- `expected: Vec<String>`: a list of item names that where expected at this index.

## Macro options

### Root attributes

- `located=<bool>`: on a structure, indicates whether or not the `location` field should be generated. Default: `true`.
- `cascade=<bool>`: if `true` on a structure, indicates that if an `Option` field is not matched, then the parser should not attempt to match other `Option` fields. It does not invalidate the overall struct parsing. Default: `false`.
- `name=<string>`: indicates the name of the struct or enum, which is used in when a parsing error occurs. Default: the name of the struct or enum.

```rust
#[parsable(located=false)] // The `location` field will not be added
struct Operation {
    first_operand: Operand,
    other_operands: Vec<(Operator, Operand)>
}
```

### Field attributes

- `prefix=<string>`: attempt to parse the specified string before parsing the field. If the prefix parsing fails, then the field parsing fails.
- `suffix=<string>`: attempt to parse the specified string after parsing the field. If the suffix parsing fails, then the field parsing fails.
- `brackets=<string>`: shortcut to specify both a prefix and a suffix using the first two characters of the specified string.
- `exclude=<string>`: indicates that the parsing is only valid if the item does not match the specified regex
- `followed_by=<string>`: indicates that the parsing if only valid if the item is followed by the specified regex.
- `not_followed_by=<string>`: indicates that the parsing if only valid if the item is not followed by the specified regex.
- `value=<string>`: on a `String` field, indicates that the field only matches the specified string.
- `regex=<string>`: on a `String` field, indicates that the field only matches the regex with the specified pattern (using the [`regex`](https://docs.rs/regex/latest/regex/) crate).
- `separator=<string>`: on a `Vec` field, specify the separator between items.
- `min=<integer>`: on a `Vec` field, specify the minimum amount of items for the parsing to be valid.
- `cascade=false`: indicates that this field ignore the root `cascade` option

## Manually implementing the `Parsable` trait

Sometimes `#[parsable]` is not enough and you want to implement your own parsing mechanism. This is done by implementing the `parse_item`, `get_item_name` and `location` methods.

```rust
use parsable::{Parsable, StringReader};

struct MyInteger {
    value: u32,
    location: ItemLocation,
}

impl Parsable for MyInteger {
    fn parse_item(reader: &mut StringReader) -> Option<Self> {
        let start = reader.get_index();

        match reader.read_regex(r"\d+") {
            Some(string) => Some(MyInteger {
                value: string.parse().unwrap(),
                location: reader.get_item_location(start),
            }),
            None => None,
        }
    }

    // Only used in errors
    fn get_item_name() -> String {
        "integer".to_string()
    }

    // Not required, but convenient
    fn location(&self) -> &ItemLocation {
        &self.location
    }
}

fn main() {
    let number_string = "56";
    let number = MyInteger::parse(number_string.to_string(), ParseOptions::default()).unwrap();
    println!("{}", number.value);
}
```

See the `StringReader` documentation (TODO).

## License

MIT