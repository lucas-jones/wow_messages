# AuraLog

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:242`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L242).
```rust,ignore
struct AuraLog {
    AuraType aura_type;
    if (aura_type == PERIODIC_DAMAGE
        || aura_type == PERIODIC_DAMAGE_PERCENT) {
        u32 damage1;
        SpellSchool school;
        u32 absorbed;
        u32 resisted;
    }
    else if (aura_type == PERIODIC_HEAL
        || aura_type == OBS_MOD_HEALTH) {
        u32 damage2;
    }
    else if (aura_type == OBS_MOD_MANA
        || aura_type == PERIODIC_ENERGIZE) {
        u32 misc_value1;
        u32 damage3;
    }
    else if (aura_type == PERIODIC_MANA_LEECH) {
        u32 misc_value2;
        u32 damage;
        f32 gain_multiplier;
    }
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 4 / - | [AuraType](auratype.md) | aura_type |  |

If aura_type is equal to `PERIODIC_DAMAGE` **or** 
is equal to `PERIODIC_DAMAGE_PERCENT`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | 4 / Little | u32 | damage1 |  |
| 0x08 | 1 / - | [SpellSchool](spellschool.md) | school |  |
| 0x09 | 4 / Little | u32 | absorbed |  |
| 0x0D | 4 / Little | u32 | resisted | vmangos: Sent as int32 |

Else If aura_type is equal to `PERIODIC_HEAL` **or** 
is equal to `OBS_MOD_HEALTH`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x11 | 4 / Little | u32 | damage2 |  |

Else If aura_type is equal to `OBS_MOD_MANA` **or** 
is equal to `PERIODIC_ENERGIZE`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x15 | 4 / Little | u32 | misc_value1 | vmangos: A miscvalue that is dependent on what the aura will do, this is usually decided by the AuraType, ie: with AuraType::SPELL_AURA_MOD_BASE_RESISTANCE_PCT this value could be SpellSchoolMask::SPELL_SCHOOL_MASK_NORMAL which would tell the aura that it should change armor.  If Modifier::m_auraname would have been AuraType::SPELL_AURA_MOUNTED then m_miscvalue would have decided which model the mount should have |
| 0x19 | 4 / Little | u32 | damage3 |  |

Else If aura_type is equal to `PERIODIC_MANA_LEECH`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x1D | 4 / Little | u32 | misc_value2 | vmangos: A miscvalue that is dependent on what the aura will do, this is usually decided by the AuraType, ie: with AuraType::SPELL_AURA_MOD_BASE_RESISTANCE_PCT this value could be SpellSchoolMask::SPELL_SCHOOL_MASK_NORMAL which would tell the aura that it should change armor.  If Modifier::m_auraname would have been AuraType::SPELL_AURA_MOUNTED then m_miscvalue would have decided which model the mount should have |
| 0x21 | 4 / Little | u32 | damage |  |
| 0x25 | 4 / Little | f32 | gain_multiplier |  |


Used in:
* [SMSG_PERIODICAURALOG](smsg_periodicauralog.md)

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:579`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L579).
```rust,ignore
struct AuraLog {
    AuraType aura_type;
    if (aura_type == PERIODIC_DAMAGE
        || aura_type == PERIODIC_DAMAGE_PERCENT) {
        u32 damage1;
        SpellSchool school;
        u32 absorbed;
        u32 resisted;
    }
    else if (aura_type == PERIODIC_HEAL
        || aura_type == OBS_MOD_HEALTH) {
        u32 damage2;
    }
    else if (aura_type == OBS_MOD_MANA
        || aura_type == PERIODIC_ENERGIZE) {
        u32 misc_value1;
        u32 damage3;
    }
    else if (aura_type == PERIODIC_MANA_LEECH) {
        u32 misc_value2;
        u32 damage;
        f32 gain_multiplier;
    }
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 4 / - | [AuraType](auratype.md) | aura_type |  |

If aura_type is equal to `PERIODIC_DAMAGE` **or** 
is equal to `PERIODIC_DAMAGE_PERCENT`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | 4 / Little | u32 | damage1 |  |
| 0x08 | 1 / - | [SpellSchool](spellschool.md) | school |  |
| 0x09 | 4 / Little | u32 | absorbed |  |
| 0x0D | 4 / Little | u32 | resisted | vmangos: Sent as int32 |

Else If aura_type is equal to `PERIODIC_HEAL` **or** 
is equal to `OBS_MOD_HEALTH`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x11 | 4 / Little | u32 | damage2 |  |

Else If aura_type is equal to `OBS_MOD_MANA` **or** 
is equal to `PERIODIC_ENERGIZE`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x15 | 4 / Little | u32 | misc_value1 | vmangos: A miscvalue that is dependent on what the aura will do, this is usually decided by the AuraType, ie: with AuraType::SPELL_AURA_MOD_BASE_RESISTANCE_PCT this value could be SpellSchoolMask::SPELL_SCHOOL_MASK_NORMAL which would tell the aura that it should change armor.  If Modifier::m_auraname would have been AuraType::SPELL_AURA_MOUNTED then m_miscvalue would have decided which model the mount should have |
| 0x19 | 4 / Little | u32 | damage3 |  |

Else If aura_type is equal to `PERIODIC_MANA_LEECH`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x1D | 4 / Little | u32 | misc_value2 | vmangos: A miscvalue that is dependent on what the aura will do, this is usually decided by the AuraType, ie: with AuraType::SPELL_AURA_MOD_BASE_RESISTANCE_PCT this value could be SpellSchoolMask::SPELL_SCHOOL_MASK_NORMAL which would tell the aura that it should change armor.  If Modifier::m_auraname would have been AuraType::SPELL_AURA_MOUNTED then m_miscvalue would have decided which model the mount should have |
| 0x21 | 4 / Little | u32 | damage |  |
| 0x25 | 4 / Little | f32 | gain_multiplier |  |


Used in:
* [SMSG_PERIODICAURALOG](smsg_periodicauralog.md)

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:956`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L956).
```rust,ignore
struct AuraLog {
    AuraType aura_type;
    if (aura_type == PERIODIC_DAMAGE
        || aura_type == PERIODIC_DAMAGE_PERCENT) {
        u32 damage1;
        u32 overkill_damage;
        SpellSchool school;
        u32 absorb1;
        u32 resisted;
        Bool critical1;
    }
    else if (aura_type == PERIODIC_HEAL
        || aura_type == OBS_MOD_HEALTH) {
        u32 damage2;
        u32 over_damage;
        u32 absorb2;
        Bool critical2;
    }
    else if (aura_type == OBS_MOD_POWER
        || aura_type == PERIODIC_ENERGIZE) {
        u32 misc_value1;
        u32 damage3;
    }
    else if (aura_type == PERIODIC_MANA_LEECH) {
        u32 misc_value2;
        u32 damage4;
        f32 gain_multiplier;
    }
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 4 / - | [AuraType](auratype.md) | aura_type |  |

If aura_type is equal to `PERIODIC_DAMAGE` **or** 
is equal to `PERIODIC_DAMAGE_PERCENT`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | 4 / Little | u32 | damage1 |  |
| 0x08 | 4 / Little | u32 | overkill_damage |  |
| 0x0C | 1 / - | [SpellSchool](spellschool.md) | school |  |
| 0x0D | 4 / Little | u32 | absorb1 |  |
| 0x11 | 4 / Little | u32 | resisted | vmangos: Sent as int32 |
| 0x15 | 1 / - | Bool | critical1 | new 3.1.2 critical tick |

Else If aura_type is equal to `PERIODIC_HEAL` **or** 
is equal to `OBS_MOD_HEALTH`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x16 | 4 / Little | u32 | damage2 |  |
| 0x1A | 4 / Little | u32 | over_damage |  |
| 0x1E | 4 / Little | u32 | absorb2 |  |
| 0x22 | 1 / - | Bool | critical2 | new 3.1.2 critical tick |

Else If aura_type is equal to `OBS_MOD_POWER` **or** 
is equal to `PERIODIC_ENERGIZE`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x23 | 4 / Little | u32 | misc_value1 | vmangos: A miscvalue that is dependent on what the aura will do, this is usually decided by the AuraType, ie: with AuraType::SPELL_AURA_MOD_BASE_RESISTANCE_PCT this value could be SpellSchoolMask::SPELL_SCHOOL_MASK_NORMAL which would tell the aura that it should change armor.  If Modifier::m_auraname would have been AuraType::SPELL_AURA_MOUNTED then m_miscvalue would have decided which model the mount should have |
| 0x27 | 4 / Little | u32 | damage3 |  |

Else If aura_type is equal to `PERIODIC_MANA_LEECH`:

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x2B | 4 / Little | u32 | misc_value2 | vmangos: A miscvalue that is dependent on what the aura will do, this is usually decided by the AuraType, ie: with AuraType::SPELL_AURA_MOD_BASE_RESISTANCE_PCT this value could be SpellSchoolMask::SPELL_SCHOOL_MASK_NORMAL which would tell the aura that it should change armor.  If Modifier::m_auraname would have been AuraType::SPELL_AURA_MOUNTED then m_miscvalue would have decided which model the mount should have |
| 0x2F | 4 / Little | u32 | damage4 |  |
| 0x33 | 4 / Little | f32 | gain_multiplier |  |


Used in:
* [SMSG_PERIODICAURALOG](smsg_periodicauralog.md)

