#![allow(non_snake_case)]

pub mod components;
pub mod pages;

use rusqlite::Connection;
use std::fmt::{Display, Formatter};

pub fn initialize_db() -> Connection {
    Connection::open("pokemon.db").expect("Failed to get DB file")
}

#[derive(Clone)]
pub struct Pokemon {
    id: usize,
    pokemon: String,
    main_format: String,
    set_format: String,
    set_name: String,
    item: String,
    ability: String,
    nature: String,
    evs: String,
    ivs: String,
    moves: String,
    complete: bool,
}

// pub async fn initialize_db_data(conn: &Connection) {
//     conn.execute(
//         "
//     create table pkm (
//     id          integer primary key,
//     pokemon     varchar(20),
//     main_format varchar(4),
//     set_format  varchar(4),
//     set_name    text,
//     item        text,
//     ability     text,
//     nature      text,
//     evs         text,
//     ivs         text,
//     moves       text,
//     complete    boolean default false
// );",
//         (),
//     )
//     .expect("Failed creating table");
//     let basics = Client::get_basics(Generation::DiamondPearl)
//         .await
//         .expect("Failed to get gen 4 basics");
//     let pokemon = basics
//         .pokemon
//         .into_iter()
//         .filter_map(|bp| {
//             if !bp.formats.is_empty() && bp.is_non_standard.as_str() == "Standard" {
//                 Some((bp.name, bp.formats.into_iter().next().unwrap()))
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<_>>();
//
//     for chunk in pokemon.chunks(5) {
//         'inner: for (pokemon, main_format) in chunk {
//             if let Ok(pokemon_dump) = Client::get_pokemon(
//                 Generation::DiamondPearl,
//                 pokemon
//                     .to_lowercase()
//                     .replace(' ', "-")
//                     .replace('\'', "")
//                     .replace('.', ""),
//             )
//             .await
//             {
//                 let mut strategies = pokemon_dump
//                     .strategies
//                     .into_iter()
//                     .filter(|s| ["OU", "UU", "NU", "RU", "Uber"].contains(&s.format.as_str()))
//                     .collect::<Vec<_>>();
//                 strategies.sort_by(|a, b| {
//                     Format::from(a.format.as_str()).cmp(&Format::from(b.format.as_str()))
//                 });
//                 if strategies.is_empty() {
//                     println!("No strategies for {}", pokemon);
//                     continue 'inner;
//                 }
//                 let strat = strategies.last().unwrap();
//                 let set_format = &strat.format;
//
//                 let move_set = strat.move_sets.first().unwrap();
//
//                 let set_name = &move_set.name;
//                 let item = move_set.items.first().unwrap();
//                 let ability = move_set
//                     .abilities
//                     .first()
//                     .map(|a| a.to_string())
//                     .unwrap_or_default();
//                 let nature = move_set.natures.first().unwrap();
//                 let evs = move_set
//                     .get_ev_configs()
//                     .split(" | ")
//                     .next()
//                     .unwrap()
//                     .to_string();
//                 let ivs = move_set
//                     .get_iv_configs()
//                     .split(" | ")
//                     .next()
//                     .map(|s| s.to_string())
//                     .unwrap_or_default();
//                 let moves = move_set
//                     .move_slots
//                     .iter()
//                     .map(|moves| {
//                         moves
//                             .iter()
//                             .map(|m| {
//                                 if let Some(mt) = m.move_type.as_ref() {
//                                     format!("- {} {}", m.move_name, mt)
//                                 } else {
//                                     format!("- {}", m.move_name)
//                                 }
//                             })
//                             .next()
//                             .unwrap()
//                     })
//                     .collect::<Vec<_>>()
//                     .join("\n");
//                 println!("Inserting {}", pokemon);
//                 conn.execute("INSERT INTO pkm (pokemon, main_format, set_format, set_name, item, ability, nature, evs, ivs, moves)
//                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)", (pokemon, main_format, set_format, set_name, item, ability, nature, evs, ivs, moves)).expect("Failed to insert into db");
//             } else {
//                 println!("Failed to get {}", pokemon);
//             }
//         }
//         thread::sleep(Duration::from_secs(3));
//     }
// }

pub fn get_pokemon_list(format: Format, conn: &Connection) -> Vec<(usize, String, bool)> {
    let mut stmt = conn
        .prepare("SELECT id, pokemon, complete from pkm where main_format = ?")
        .expect("Failed to prepare statement");
    stmt.query_map([format.to_string()], |row| {
        Ok((
            row.get(0).unwrap(),
            row.get(1).unwrap(),
            row.get(2).unwrap(),
        ))
    })
    .expect("Failed to query statement")
    .map(|r| r.unwrap())
    .collect::<Vec<_>>()
}

pub fn get_pokemon(id: usize, conn: &Connection) -> Pokemon {
    let mut stmt = conn
        .prepare("SELECT * from pkm where id = ?")
        .expect("Failed to prepare statement");
    stmt.query_row([id], |row| {
        Ok(Pokemon {
            id: row.get(0).unwrap(),
            pokemon: row.get(1).unwrap(),
            main_format: row.get(2).unwrap(),
            set_format: row.get(3).unwrap(),
            set_name: row.get(4).unwrap(),
            item: row.get(5).unwrap(),
            ability: row.get(6).unwrap(),
            nature: row.get(7).unwrap(),
            evs: row.get(8).unwrap(),
            ivs: row.get(9).unwrap(),
            moves: row.get(10).unwrap(),
            complete: row.get(11).unwrap(),
        })
    })
    .expect("Failed to query statement")
}

pub fn set_complete(id: usize, conn: &Connection) {
    conn.execute("UPDATE pkm SET complete = true WHERE id = ?", [id])
        .expect("Failed to set complete");
}

pub fn set_incomplete(id: usize, conn: &Connection) {
    conn.execute("UPDATE pkm SET complete = false WHERE id = ?", [id])
        .expect("Failed to set complete");
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Format {
    None,
    Uber,
    UUBL,
    NUBL,
    NFE,
    LC,
    NU,
    RU,
    UU,
    OU,
}

impl From<&str> for Format {
    fn from(value: &str) -> Self {
        match value {
            "OU" => Format::OU,
            "NU" => Format::NU,
            "RU" => Format::RU,
            "UU" => Format::UU,
            "Uber" => Format::Uber,
            "UUBL" => Format::UUBL,
            "NUBL" => Format::NUBL,
            "NFE" => Format::NFE,
            "LC" => Format::LC,
            _ => Format::None,
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::None => write!(f, "None"),
            Format::Uber => write!(f, "Uber"),
            Format::NU => write!(f, "NU"),
            Format::RU => write!(f, "RU"),
            Format::UU => write!(f, "UU"),
            Format::OU => write!(f, "OU"),
            Format::UUBL => write!(f, "UUBL"),
            Format::NUBL => write!(f, "NUBL"),
            Format::NFE => write!(f, "NFE"),
            Format::LC => write!(f, "LC"),
        }
    }
}
