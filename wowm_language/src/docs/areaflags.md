# AreaFlags

## Client Version 1.12

Used in `AreaTable.dbc`.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/external/area_flags.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/area_flags.wowm#L2).

```rust,ignore
flag AreaFlags : u16 {
    SNOW = 0x01;
    UNK = 0x02;
    DEVELOPMENT = 0x04;
    UNK2 = 0x08;
    UNK3 = 0x10;
    CITY_SLAVE = 0x20;
    CITY_ALLOW_DUELS = 0x40;
    UNK4 = 0x80;
    CITY = 0x100;
    TEST = 0x200;
}
```
### Type
The basic type is `u16`, a 2 byte (16 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Comment |
| --------- | -------- | ------- |
| `SNOW` | 1 (0x01) |  |
| `UNK` | 2 (0x02) |  |
| `DEVELOPMENT` | 4 (0x04) |  |
| `UNK2` | 8 (0x08) |  |
| `UNK3` | 16 (0x10) |  |
| `CITY_SLAVE` | 32 (0x20) |  |
| `CITY_ALLOW_DUELS` | 64 (0x40) |  |
| `UNK4` | 128 (0x80) |  |
| `CITY` | 256 (0x100) |  |
| `TEST` | 512 (0x200) |  |

Used in:
