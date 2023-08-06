# rustbasics
Rust basics

Rust Enum
- Rust enumeration data type allows you to select a value from a list of possible variants.

Syntax/Declaration
................................................................
enum enum_name {
    variant1,
    variant2,
    variant3
}

Example
................................................................
enum Status {
    Pending,
    Inprogress,
    Active,
    Closed,
}

To avoid trait std::fmt::Debug error messages, add attribute #[derive(Debug)] to the beginning of the enum.

i.e

#[derive(Debug)]
enum Status {
    Pending
}
