# SMSG_INSTANCE_RESET_FAILED

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm#L13).
```rust,ignore
smsg SMSG_INSTANCE_RESET_FAILED = 0x031F {
    (u32)InstanceResetFailedReason reason;
    Map map;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | 4 / - | [InstanceResetFailedReason](instanceresetfailedreason.md) | reason |  |
| 0x08 | 4 / - | [Map](map.md) | map |  |

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm#L13).
```rust,ignore
smsg SMSG_INSTANCE_RESET_FAILED = 0x031F {
    (u32)InstanceResetFailedReason reason;
    Map map;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | 4 / - | [InstanceResetFailedReason](instanceresetfailedreason.md) | reason |  |
| 0x08 | 4 / - | [Map](map.md) | map |  |

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm#L13).
```rust,ignore
smsg SMSG_INSTANCE_RESET_FAILED = 0x031F {
    (u32)InstanceResetFailedReason reason;
    Map map;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | 4 / - | [InstanceResetFailedReason](instanceresetfailedreason.md) | reason |  |
| 0x08 | 4 / - | [Map](map.md) | map |  |

