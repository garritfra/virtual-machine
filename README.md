## Usage

```bash
cargo run ./example.dasm
```

## Op-Codes

| Instruction | Bytes | Description                                        |
| :---------- | :---- | :------------------------------------------------- |
| nop         | 0x00  | Do nothing                                         |
| halt        | 0x01  | Stop execution                                     |
| push        | 0x02  | Push the next token onto the stack                 |
| pop         | 0x03  | Push the next token onto the stack                 |
| mov         | 0x04  | Pops a value off the stack and prints it to stdout |
| add         | 0x05  | Add two values on top of the stack               |

# Assembly Reference

A `value` is an unsigned 8 bit value represented by a decimal number:

```
42
0
```

A `register` is the name of one of the available registers:

```
r0
r15
```

An `address` is the location of an absolute address in memory, prefixed with a
`#` symbol:

```
#0000
#1000
```

### `nop`

Do nothing and continue with next operation.

Syntax:

```
nop
```

### `halt`

Halt the execution of the program

Syntax:

```
halt
```

### `push`

Push the value of the next token onto the stack

Syntax:

```
push <value>
push <register>
push <address>
```

Examples:

```
push 42
push r0
push #1000
```

### `pop`

Pops a value of the stack and into the given location

Syntax:

```
pop <register>
pop <address>
```

Examples:

```
pop r0
pop #0001
```

### `mov`

The mov instruction copies the data item referred to by its second operand
(i.e. register contents or a constant value) into the
location referred to by its first operand (i.e. a register or memory)

Syntax:

```
mov <register> <register>
mov <register> <address>
mov <address> <register>
mov <register> <value>
mov <address> <value>
```

Examples:

```
mov r0 r1
mov r0 42
```

### `add`

Pops the two values on the stack and places the result back on the stack

Syntax:

```
add
```

