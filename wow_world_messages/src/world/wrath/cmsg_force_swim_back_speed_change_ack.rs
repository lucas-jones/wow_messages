use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MovementFlags, MovementInfo, TransportInfo, Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_force_swim_back_speed_change_ack.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_force_swim_back_speed_change_ack.wowm#L19):
/// ```text
/// cmsg CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK = 0x02DD {
///     PackedGuid guid;
///     u32 counter;
///     MovementInfo info;
///     f32 new_speed;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub info: MovementInfo,
    pub new_speed: f32,
}

impl crate::private::Sealed for CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {}
impl CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(39..=105).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            counter,
            info,
            new_speed,
        })
    }

}

impl crate::Message for CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    const OPCODE: u32 = 0x02dd;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    counter = {};", self.counter).unwrap();
        // info: MovementInfo
        writeln!(s, "    info = {{").unwrap();
        // Members
        writeln!(s, "        flags = {};", MovementFlags::new(self.info.flags.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "        timestamp = {};", self.info.timestamp).unwrap();
        // position: Vector3d
        writeln!(s, "        position = {{").unwrap();
        // Members
        writeln!(s, "            x = {};", if self.info.position.x.to_string().contains('.') { self.info.position.x.to_string() } else { format!("{}.0", self.info.position.x) }).unwrap();
        writeln!(s, "            y = {};", if self.info.position.y.to_string().contains('.') { self.info.position.y.to_string() } else { format!("{}.0", self.info.position.y) }).unwrap();
        writeln!(s, "            z = {};", if self.info.position.z.to_string().contains('.') { self.info.position.z.to_string() } else { format!("{}.0", self.info.position.z) }).unwrap();

        writeln!(s, "        }};").unwrap();
        writeln!(s, "        orientation = {};", if self.info.orientation.to_string().contains('.') { self.info.orientation.to_string() } else { format!("{}.0", self.info.orientation) }).unwrap();
        if let Some(if_statement) = &self.info.flags.get_on_transport_and_interpolated_movement() {
            match if_statement {
                crate::wrath::MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                    transport_info,
                    transport_time,
                } => {
                    // transport_info: TransportInfo
                    writeln!(s, "        transport_info = {{").unwrap();
                    // Members
                    writeln!(s, "            guid = {};", transport_info.guid.guid()).unwrap();
                    // position: Vector3d
                    writeln!(s, "            position = {{").unwrap();
                    // Members
                    writeln!(s, "                x = {};", if transport_info.position.x.to_string().contains('.') { transport_info.position.x.to_string() } else { format!("{}.0", transport_info.position.x) }).unwrap();
                    writeln!(s, "                y = {};", if transport_info.position.y.to_string().contains('.') { transport_info.position.y.to_string() } else { format!("{}.0", transport_info.position.y) }).unwrap();
                    writeln!(s, "                z = {};", if transport_info.position.z.to_string().contains('.') { transport_info.position.z.to_string() } else { format!("{}.0", transport_info.position.z) }).unwrap();

                    writeln!(s, "            }};").unwrap();
                    writeln!(s, "            orientation = {};", if transport_info.orientation.to_string().contains('.') { transport_info.orientation.to_string() } else { format!("{}.0", transport_info.orientation) }).unwrap();
                    writeln!(s, "            timestamp = {};", transport_info.timestamp).unwrap();
                    writeln!(s, "            seat = {};", transport_info.seat).unwrap();

                    writeln!(s, "        }};").unwrap();
                    writeln!(s, "        transport_time = {};", transport_time).unwrap();
                }
                crate::wrath::MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                    transport,
                } => {
                    // transport: TransportInfo
                    writeln!(s, "        transport = {{").unwrap();
                    // Members
                    writeln!(s, "            guid = {};", transport.guid.guid()).unwrap();
                    // position: Vector3d
                    writeln!(s, "            position = {{").unwrap();
                    // Members
                    writeln!(s, "                x = {};", if transport.position.x.to_string().contains('.') { transport.position.x.to_string() } else { format!("{}.0", transport.position.x) }).unwrap();
                    writeln!(s, "                y = {};", if transport.position.y.to_string().contains('.') { transport.position.y.to_string() } else { format!("{}.0", transport.position.y) }).unwrap();
                    writeln!(s, "                z = {};", if transport.position.z.to_string().contains('.') { transport.position.z.to_string() } else { format!("{}.0", transport.position.z) }).unwrap();

                    writeln!(s, "            }};").unwrap();
                    writeln!(s, "            orientation = {};", if transport.orientation.to_string().contains('.') { transport.orientation.to_string() } else { format!("{}.0", transport.orientation) }).unwrap();
                    writeln!(s, "            timestamp = {};", transport.timestamp).unwrap();
                    writeln!(s, "            seat = {};", transport.seat).unwrap();

                    writeln!(s, "        }};").unwrap();
                }
            }
        }

        if let Some(if_statement) = &self.info.flags.get_swimming() {
            match if_statement {
                crate::wrath::MovementInfo_MovementFlags_Swimming::Swimming {
                    pitch1,
                } => {
                    writeln!(s, "        pitch1 = {};", if pitch1.to_string().contains('.') { pitch1.to_string() } else { format!("{}.0", pitch1) }).unwrap();
                }
                crate::wrath::MovementInfo_MovementFlags_Swimming::Flying {
                    pitch2,
                } => {
                    writeln!(s, "        pitch2 = {};", if pitch2.to_string().contains('.') { pitch2.to_string() } else { format!("{}.0", pitch2) }).unwrap();
                }
                crate::wrath::MovementInfo_MovementFlags_Swimming::AlwaysAllowPitching {
                    pitch3,
                } => {
                    writeln!(s, "        pitch3 = {};", if pitch3.to_string().contains('.') { pitch3.to_string() } else { format!("{}.0", pitch3) }).unwrap();
                }
            }
        }

        writeln!(s, "        fall_time = {};", if self.info.fall_time.to_string().contains('.') { self.info.fall_time.to_string() } else { format!("{}.0", self.info.fall_time) }).unwrap();
        if let Some(if_statement) = &self.info.flags.get_falling() {
            writeln!(s, "        z_speed = {};", if if_statement.z_speed.to_string().contains('.') { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
            writeln!(s, "        cos_angle = {};", if if_statement.cos_angle.to_string().contains('.') { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
            writeln!(s, "        sin_angle = {};", if if_statement.sin_angle.to_string().contains('.') { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
            writeln!(s, "        xy_speed = {};", if if_statement.xy_speed.to_string().contains('.') { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
        }

        if let Some(if_statement) = &self.info.flags.get_spline_elevation() {
            writeln!(s, "        spline_elevation = {};", if if_statement.spline_elevation.to_string().contains('.') { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
        }


        writeln!(s, "    }};").unwrap();
        writeln!(s, "    new_speed = {};", if self.new_speed.to_string().contains('.') { self.new_speed.to_string() } else { format!("{}.0", self.new_speed) }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 733_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "counter", "    ");
        writeln!(s, "    /* info: MovementInfo start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 6, "flags", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "        ");
        writeln!(s, "    /* position: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "            ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "            ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "            ");
        writeln!(s, "    /* position: Vector3d end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "        ");
        if let Some(if_statement) = &self.info.flags.get_on_transport_and_interpolated_movement() {
            match if_statement {
                crate::wrath::MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                    transport_info,
                    transport_time,
                } => {
                    writeln!(s, "    /* transport_info: TransportInfo start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport_info.guid), "guid", "            ");
                    writeln!(s, "    /* position: Vector3d start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                    writeln!(s, "    /* position: Vector3d end */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "            ");
                    writeln!(s, "    /* transport_info: TransportInfo end */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_time", "        ");
                }
                crate::wrath::MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                    transport,
                } => {
                    writeln!(s, "    /* transport: TransportInfo start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport.guid), "guid", "            ");
                    writeln!(s, "    /* position: Vector3d start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                    writeln!(s, "    /* position: Vector3d end */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "            ");
                    writeln!(s, "    /* transport: TransportInfo end */").unwrap();
                }
            }
        }

        if let Some(if_statement) = &self.info.flags.get_swimming() {
            match if_statement {
                crate::wrath::MovementInfo_MovementFlags_Swimming::Swimming {
                    pitch1,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch1", "        ");
                }
                crate::wrath::MovementInfo_MovementFlags_Swimming::Flying {
                    pitch2,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch2", "        ");
                }
                crate::wrath::MovementInfo_MovementFlags_Swimming::AlwaysAllowPitching {
                    pitch3,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch3", "        ");
                }
            }
        }

        crate::util::write_bytes(&mut s, &mut bytes, 4, "fall_time", "        ");
        if let Some(if_statement) = &self.info.flags.get_falling() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "z_speed", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "cos_angle", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "sin_angle", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "xy_speed", "        ");
        }

        if let Some(if_statement) = &self.info.flags.get_spline_elevation() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_elevation", "        ");
        }

        writeln!(s, "    /* info: MovementInfo end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "new_speed", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(733, "CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {}

impl CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // new_speed: f32
    }
}

