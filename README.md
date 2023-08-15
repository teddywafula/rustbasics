# rustbasics

## DECISION MAKING

- Normally we check a condition if it meets certain criteria then we execute a statement.
  We have:
- if statement
- if else statement
- else if and nested if statements
- match or switch statement

## Examples

### Using if statement
  if variable condition {
    print statement
  }

### Using if else statement
if variable condition {
    print statement
} else {
    print statement
}

### Using if else nested condition

if variable condition {

} else if variable condition {

} else {

}


### Using match

let variable = value

let result = match variable {
    value1 => return result,
    value2 => return result2,
    _ => return defaultValue
};

print result