## Usage

```bash
cargo run ./example.dasm
```

## Op-Codes

| Instruction | Bytes | Description                                        |
| :---------- | :---- | :------------------------------------------------- |
| nop         | 0x00  | Do nothing                                         |
| halt        | 0xff  | Stop execution                                     |
| push        | 0x01  | Push the next token onto the stack                 |
| print       | 0x02  | Pops a value off the stack and prints it to stdout |
| add         | 0x03  | Add two integers on top of the stack               |
