use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_wrap_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_wrap_item.wowm#L3):
/// ```text
/// cmsg CMSG_WRAP_ITEM = 0x01D3 {
///     u8 gift_bag_index;
///     u8 gift_slot;
///     u8 item_bag_index;
///     u8 item_slot;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_WRAP_ITEM {
    pub gift_bag_index: u8,
    pub gift_slot: u8,
    pub item_bag_index: u8,
    pub item_slot: u8,
}

impl crate::private::Sealed for CMSG_WRAP_ITEM {}
impl CMSG_WRAP_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // gift_bag_index: u8
        let gift_bag_index = crate::util::read_u8_le(&mut r)?;

        // gift_slot: u8
        let gift_slot = crate::util::read_u8_le(&mut r)?;

        // item_bag_index: u8
        let item_bag_index = crate::util::read_u8_le(&mut r)?;

        // item_slot: u8
        let item_slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            gift_bag_index,
            gift_slot,
            item_bag_index,
            item_slot,
        })
    }

}

impl crate::Message for CMSG_WRAP_ITEM {
    const OPCODE: u32 = 0x01d3;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_WRAP_ITEM"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_WRAP_ITEM {{").unwrap();
        // Members
        writeln!(s, "    gift_bag_index = {};", self.gift_bag_index).unwrap();
        writeln!(s, "    gift_slot = {};", self.gift_slot).unwrap();
        writeln!(s, "    item_bag_index = {};", self.item_bag_index).unwrap();
        writeln!(s, "    item_slot = {};", self.item_slot).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 467_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "gift_bag_index", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "gift_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "item_bag_index", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "item_slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // gift_bag_index: u8
        w.write_all(&self.gift_bag_index.to_le_bytes())?;

        // gift_slot: u8
        w.write_all(&self.gift_slot.to_le_bytes())?;

        // item_bag_index: u8
        w.write_all(&self.item_bag_index.to_le_bytes())?;

        // item_slot: u8
        w.write_all(&self.item_slot.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(467, "CMSG_WRAP_ITEM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_WRAP_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_WRAP_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_WRAP_ITEM {}

