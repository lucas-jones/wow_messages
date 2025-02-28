# CMD_REALM_LIST_Client

## Protocol Version 2, Protocol Version 3, Protocol Version 5, Protocol Version 6, Protocol Version 7, Protocol Version 8

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/login/cmd_realm/client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/client.wowm#L3).
```rust,ignore
clogin CMD_REALM_LIST_Client = 0x10 {
    u32 padding = 0;
}
```
### Header

Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x01 | 4 / Little | u32 | padding |  |

### Examples

#### Example 1

```c
16, // opcode (16)
0, 0, 0, 0, // padding: u32
```
