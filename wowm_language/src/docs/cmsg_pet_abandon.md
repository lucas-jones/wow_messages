# CMSG_PET_ABANDON

## Client Version 1, Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/pet/cmsg_pet_abandon.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_abandon.wowm#L3).
```rust,ignore
cmsg CMSG_PET_ABANDON = 0x0176 {
    Guid pet;
}
```
### Header

CMSG have a header of 6 bytes.

#### CMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x06 | 8 / Little | [Guid](../types/packed-guid.md) | pet |  |

