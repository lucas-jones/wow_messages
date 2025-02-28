# CMSG_MAIL_TAKE_ITEM

## Client Version 1

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/mail/cmsg_mail_take_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_mail_take_item.wowm#L1).
```rust,ignore
cmsg CMSG_MAIL_TAKE_ITEM = 0x0246 {
    Guid mailbox;
    u32 mail_id;
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
| 0x06 | 8 / Little | [Guid](../types/packed-guid.md) | mailbox |  |
| 0x0E | 4 / Little | u32 | mail_id |  |

## Client Version 2.4.3, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/mail/cmsg_mail_take_item.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_mail_take_item.wowm#L8).
```rust,ignore
cmsg CMSG_MAIL_TAKE_ITEM = 0x0246 {
    Guid mailbox;
    u32 mail_id;
    Item item;
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
| 0x06 | 8 / Little | [Guid](../types/packed-guid.md) | mailbox |  |
| 0x0E | 4 / Little | u32 | mail_id |  |
| 0x12 | 4 / Little | Item | item |  |

