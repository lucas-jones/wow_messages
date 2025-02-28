# SMSG_LOGOUT_RESPONSE

## Client Version 1.12, Client Version 2, Client Version 3

Reply to [CMSG_LOGOUT_REQUEST](./cmsg_logout_request.md).

The client expects to get an [SMSG_LOGOUT_COMPLETE](./smsg_logout_complete.md) when logout is complete.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm#L18).
```rust,ignore
smsg SMSG_LOGOUT_RESPONSE = 0x004C {
    LogoutResult result;
    LogoutSpeed speed;
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
| 0x04 | 4 / - | [LogoutResult](logoutresult.md) | result |  |
| 0x08 | 1 / - | [LogoutSpeed](logoutspeed.md) | speed |  |

### Examples

#### Example 1

```c
0, 7, // size
76, 0, // opcode (76)
0, 0, 0, 0, // result: LogoutResult SUCCESS (0)
1, // speed: LogoutSpeed INSTANT (1)
```
