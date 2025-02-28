use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use wow_srp::normalized_string::NormalizedString;
use wow_srp::server::SrpServer;
use wow_srp::tbc_header::ProofSeed;
use wow_world_messages::tbc::opcodes::ClientOpcodeMessage;
use wow_world_messages::tbc::tokio_expect_client_message;
use wow_world_messages::tbc::ServerMessage;
use wow_world_messages::tbc::*;
use wow_world_messages::tbc::{UpdateMask, UpdatePlayer};
use wow_world_messages::Guid;

pub async fn world(users: Arc<Mutex<HashMap<String, SrpServer>>>) {
    let listener = TcpListener::bind("0.0.0.0:8085").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(handle(stream, users.clone()));
    }
}

async fn handle(mut stream: TcpStream, users: Arc<Mutex<HashMap<String, SrpServer>>>) {
    let seed = ProofSeed::new();

    SMSG_AUTH_CHALLENGE {
        server_seed: seed.seed(),
    }
    .tokio_write_unencrypted_server(&mut stream)
    .await
    .unwrap();

    let c = tokio_expect_client_message::<CMSG_AUTH_SESSION, _>(&mut stream)
        .await
        .unwrap();

    let session_key = {
        let mut server = users.lock().unwrap();
        *server.get_mut(&c.username).unwrap().session_key()
    };

    let mut encryption = seed
        .into_server_header_crypto(
            &NormalizedString::new(&c.username).unwrap(),
            session_key,
            c.client_proof,
            c.client_seed,
        )
        .unwrap();

    SMSG_AUTH_RESPONSE::AuthOk {
        billing_flags: BillingPlanFlags::empty(),
        billing_rested: 0,
        billing_time: 0,
        expansion: Expansion::TheBurningCrusade,
    }
    .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
    .await
    .unwrap();

    loop {
        let opcode =
            ClientOpcodeMessage::tokio_read_encrypted(&mut stream, encryption.decrypter()).await;

        let opcode = match opcode {
            Ok(e) => e,
            Err(e) => {
                dbg!(e);
                continue;
            }
        };

        match opcode {
            ClientOpcodeMessage::CMSG_PING(c) => {
                SMSG_PONG {
                    sequence_id: c.sequence_id,
                }
                .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
                .await
                .unwrap();
            }
            ClientOpcodeMessage::CMSG_CHAR_ENUM => {
                SMSG_CHAR_ENUM {
                    characters: vec![Character {
                        guid: Guid::new(4),
                        name: "Warr".to_string(),
                        race: Race::Human,
                        class: Class::Warrior,
                        gender: Gender::Female,
                        skin: 0,
                        face: 0,
                        hair_style: 0,
                        hair_color: 0,
                        facial_hair: 0,
                        level: Level::new_player(),
                        area: Area::NorthshireValley,
                        map: Map::EasternKingdoms,
                        position: Vector3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        guild_id: 0,
                        flags: Default::default(),
                        first_login: false,
                        pet_display_id: 0,
                        pet_level: Level::zero(),
                        pet_family: CreatureFamily::None,
                        equipment: [Default::default(); 20],
                    }],
                }
                .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
                .await
                .unwrap();
            }
            ClientOpcodeMessage::CMSG_CHAR_CREATE(c) => {
                let result = match c.name.to_uppercase().as_str() {
                    "SYSTEME" => WorldResult::AuthSystemError,
                    "SERVERSH" => WorldResult::AuthServerShuttingDown,
                    "WAITQU" => WorldResult::AuthWaitQueue,
                    "ERROR" => WorldResult::CharCreateError,
                    "SERVERL" => WorldResult::CharCreateServerLimit,
                    // CCSUCC immediately returns to character screen
                    "CCSUCC" => WorldResult::CharCreateSuccess,
                    "SERVERQU" => WorldResult::CharCreateServerQueue,
                    // Above fail
                    _ => WorldResult::CharCreateError,
                };

                SMSG_CHAR_CREATE { result }
                    .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
                    .await
                    .unwrap();
            }
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(_) => {
                break;
            }
            e => {
                dbg!(e);
            }
        }
    }

    SMSG_LOGIN_VERIFY_WORLD {
        map: Map::EasternKingdoms,
        position: Vector3d {
            x: 200.0,
            y: 200.0,
            z: 200.0,
        },
        orientation: 0.0,
    }
    .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
    .await
    .unwrap();

    SMSG_TUTORIAL_FLAGS {
        tutorial_data: [
            0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
            0xFFFFFFFF,
        ],
    }
    .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
    .await
    .unwrap();

    let update_mask = UpdatePlayer::builder()
        .set_object_guid(Guid::new(4))
        .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
        .set_object_scale_x(1.0)
        .set_unit_health(100)
        .set_unit_maxhealth(100)
        .set_unit_level(1)
        .set_unit_factiontemplate(1)
        .set_unit_displayid(50)
        .set_unit_nativedisplayid(50)
        .finalize();

    let update_flag = MovementBlock_UpdateFlag::empty()
        .set_living(MovementBlock_UpdateFlag_Living::Living {
            backwards_running_speed: 4.5,
            backwards_swimming_speed: 0.0,
            extra_flags: 0,
            fall_time: 0.0,
            flags: MovementBlock_MovementFlags::empty(),
            flying_speed: 0.0,
            backwards_flying_speed: 0.0,
            living_orientation: 0.0,
            living_position: Vector3d {
                x: -8949.95,
                y: -132.493,
                z: 83.5312,
            },
            running_speed: 7.0,
            swimming_speed: 0.0,
            timestamp: 0,
            turn_rate: std::f32::consts::PI,
            walking_speed: 1.0,
        })
        .set_all(MovementBlock_UpdateFlag_All { unknown2: 0 })
        .set_self();

    SMSG_UPDATE_OBJECT {
        has_transport: 0,
        objects: vec![Object::CreateObject2 {
            guid3: Guid::new(4),
            mask2: UpdateMask::Player(update_mask),
            movement2: MovementBlock { update_flag },
            object_type: ObjectType::Player,
        }],
    }
    .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
    .await
    .unwrap();

    SMSG_TIME_SYNC_REQ { time_sync: 0 }
        .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
        .await
        .unwrap();

    loop {
        let opcode =
            ClientOpcodeMessage::tokio_read_encrypted(&mut stream, encryption.decrypter()).await;
        match opcode {
            Ok(e) => {
                dbg!(e);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
}
