use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_learned_spell.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_learned_spell.wowm#L7):
/// ```text
/// smsg SMSG_LEARNED_SPELL = 0x012B {
///     Spell id;
///     u16 unknown;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_LEARNED_SPELL {
    pub id: u32,
    /// mangostwo: 3.3.3 unk
    pub unknown: u16,
}

impl crate::private::Sealed for SMSG_LEARNED_SPELL {}
impl SMSG_LEARNED_SPELL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 6 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // id: Spell
        let id = crate::util::read_u32_le(&mut r)?;

        // unknown: u16
        let unknown = crate::util::read_u16_le(&mut r)?;

        Ok(Self {
            id,
            unknown,
        })
    }

}

impl crate::Message for SMSG_LEARNED_SPELL {
    const OPCODE: u32 = 0x012b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LEARNED_SPELL"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LEARNED_SPELL {{").unwrap();
        // Members
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 299_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "unknown", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: Spell
        w.write_all(&self.id.to_le_bytes())?;

        // unknown: u16
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(299, "SMSG_LEARNED_SPELL", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LEARNED_SPELL {}

