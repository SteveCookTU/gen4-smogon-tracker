#![allow(non_snake_case)]

pub mod components;
pub mod pages;

use dioxus::hooks::Coroutine;
use rusqlite::Connection;
use smog_strat_dex_rs::{Client, Generation};
use std::fmt::{Display, Formatter};
use std::thread;
use std::time::Duration;

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
    tera_type: String,
    complete: bool,
}

pub enum InitializationMessage {
    Total(usize),
    Progress,
    End,
}

pub async fn initialize_db_data(
    conn: Connection,
    gen: Generation,
    sender: Coroutine<InitializationMessage>,
) {
    let tbl = get_table_name(gen);

    let _ = conn.execute(&format!("drop table {tbl};"), []);

    conn.execute(
        &format!(
            "
    create table {tbl} (
    id          integer primary key,
    pokemon     varchar(20),
    main_format varchar(4),
    set_format  varchar(4),
    set_name    text,
    item        text,
    ability     text,
    nature      text,
    evs         text,
    ivs         text,
    moves       text,
    tera_type   text,
    complete    boolean default false
);"
        ),
        [],
    )
    .expect("Failed creating table");
    let basics = Client::get_basics(gen).await.expect("Failed to get basics");
    let pokemon = basics
        .pokemon
        .into_iter()
        .filter_map(|bp| {
            if !bp.formats.is_empty() && bp.is_non_standard.as_str() == "Standard" {
                Some((bp.name, bp.formats.into_iter().next().unwrap()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    sender.send(InitializationMessage::Total(pokemon.len()));

    for chunk in pokemon.chunks(5) {
        'inner: for (pokemon, main_format) in chunk {
            if let Ok(pokemon_dump) = Client::get_pokemon(
                gen,
                pokemon
                    .to_lowercase()
                    .replace(' ', "-")
                    .replace(['.', '\''], ""),
            )
            .await
            {
                let mut strategies = pokemon_dump
                    .strategies
                    .into_iter()
                    .filter(|s| ["OU", "UU", "NU", "RU", "PU", "Uber"].contains(&s.format.as_str()))
                    .collect::<Vec<_>>();
                strategies.sort_by(|a, b| {
                    Format::from(a.format.as_str()).cmp(&Format::from(b.format.as_str()))
                });
                if strategies.is_empty() {
                    sender.send(InitializationMessage::Progress);
                    continue 'inner;
                }
                let strat = strategies.last().unwrap();
                let set_format = &strat.format;

                let move_set = strat.move_sets.first().unwrap();

                let set_name = &move_set.name;
                let item = move_set
                    .items
                    .first()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let ability = move_set
                    .abilities
                    .first()
                    .map(|a| a.to_string())
                    .unwrap_or_default();
                let nature = move_set
                    .natures
                    .first()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let evs = move_set
                    .get_ev_configs()
                    .split(" | ")
                    .next()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let ivs = move_set
                    .get_iv_configs()
                    .split(" | ")
                    .next()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let moves = move_set
                    .move_slots
                    .iter()
                    .map(|moves| {
                        moves
                            .iter()
                            .map(|m| {
                                if let Some(mt) = m.move_type.as_ref() {
                                    format!("- {} {}", m.move_name, mt)
                                } else {
                                    format!("- {}", m.move_name)
                                }
                            })
                            .next()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                let tera_type = move_set
                    .tera_types
                    .first()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                if conn.execute(&format!("INSERT INTO {tbl} (pokemon, main_format, set_format, set_name, item, ability, nature, evs, ivs, moves, tera_type)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"), (pokemon, main_format, set_format, set_name, item, ability, nature, evs, ivs, moves, tera_type)).is_err() {
                    println!("Error: {pokemon}");
                }
            }
            sender.send(InitializationMessage::Progress);
        }
        thread::sleep(Duration::from_secs(2));
    }

    sender.send(InitializationMessage::End);
}

pub async fn update_db_data(
    conn: Connection,
    gen: Generation,
    sender: Coroutine<InitializationMessage>,
) {
    let tbl = get_table_name(gen);

    let basics = Client::get_basics(gen).await.expect("Failed to get basics");
    let pokemon = basics
        .pokemon
        .into_iter()
        .filter_map(|bp| {
            if !bp.formats.is_empty() && bp.is_non_standard.as_str() == "Standard" {
                Some((bp.name, bp.formats.into_iter().next().unwrap()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    sender.send(InitializationMessage::Total(pokemon.len()));

    for chunk in pokemon.chunks(5) {
        'inner: for (pokemon, main_format) in chunk {
            if let Ok(pokemon_dump) = Client::get_pokemon(
                gen,
                pokemon
                    .to_lowercase()
                    .replace(' ', "-")
                    .replace(['.', '\''], ""),
            )
            .await
            {
                let mut strategies = pokemon_dump
                    .strategies
                    .into_iter()
                    .filter(|s| ["OU", "UU", "NU", "RU", "PU", "Uber"].contains(&s.format.as_str()))
                    .collect::<Vec<_>>();
                strategies.sort_by(|a, b| {
                    Format::from(a.format.as_str()).cmp(&Format::from(b.format.as_str()))
                });
                if strategies.is_empty() {
                    sender.send(InitializationMessage::Progress);
                    continue 'inner;
                }

                let strat = strategies.last().unwrap();
                let set_format = &strat.format;

                let move_set = strat.move_sets.first().unwrap();

                let set_name = &move_set.name;
                let item = move_set
                    .items
                    .first()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let ability = move_set
                    .abilities
                    .first()
                    .map(|a| a.to_string())
                    .unwrap_or_default();
                let nature = move_set
                    .natures
                    .first()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let evs = move_set
                    .get_ev_configs()
                    .split(" | ")
                    .next()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let ivs = move_set
                    .get_iv_configs()
                    .split(" | ")
                    .next()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let moves = move_set
                    .move_slots
                    .iter()
                    .map(|moves| {
                        moves
                            .iter()
                            .map(|m| {
                                if let Some(mt) = m.move_type.as_ref() {
                                    format!("- {} {}", m.move_name, mt)
                                } else {
                                    format!("- {}", m.move_name)
                                }
                            })
                            .next()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                let tera_type = move_set
                    .tera_types
                    .first()
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                let Ok(mut stmt) = conn.prepare(&format!("SELECT main_format, set_format FROM {tbl} where pokemon = \'{pokemon}\'")) else {
                    sender.send(InitializationMessage::Progress);
                    continue 'inner;
                };

                let Ok(mut query_map) = stmt.query_map([], |row| {
                    Ok((
                        row.get::<usize, String>(0).unwrap(),
                        row.get::<usize, String>(1).unwrap(),
                    ))
                }) else {
                    sender.send(InitializationMessage::Progress);
                    continue 'inner;
                };

                let stmt_str = match query_map.next() {
                    Some(Ok((orig_main, orig_set))) => {
                        if &orig_main == main_format && &orig_set == set_format {
                            sender.send(InitializationMessage::Progress);
                            continue 'inner;
                        }
                        format!("UPDATE {tbl} (pokemon = ?1, main_format = ?2, set_format = ?3, set_name = ?4, item = ?5, ability = ?6, nature = ?7, evs = ?8, ivs = ?9, moves = ?10, tera_type = ?11, complete = false) where pokemon = {pokemon}")
                    }
                    _ => {
                        format!("INSERT INTO {tbl} (pokemon, main_format, set_format, set_name, item, ability, nature, evs, ivs, moves, tera_type)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)")
                    }
                };

                if conn
                    .execute(
                        &stmt_str,
                        (
                            pokemon,
                            main_format,
                            set_format,
                            set_name,
                            item,
                            ability,
                            nature,
                            evs,
                            ivs,
                            moves,
                            tera_type,
                        ),
                    )
                    .is_err()
                {
                    println!("Error: {}", pokemon);
                }
            }
            sender.send(InitializationMessage::Progress);
        }
        thread::sleep(Duration::from_secs(2));
    }

    sender.send(InitializationMessage::End);
}

pub fn get_pokemon_list(
    format: Format,
    conn: &Connection,
    gen: Generation,
) -> Vec<(usize, String, bool)> {
    let tbl = get_table_name(gen);
    let Ok(mut stmt) = conn
        .prepare(&format!("SELECT id, pokemon, complete from {tbl} where main_format = ?")) else {
        return Vec::new();
    };
    let Ok(query_map) = stmt.query_map([format.to_string()], |row| {
        Ok((
            row.get(0).unwrap(),
            row.get(1).unwrap(),
            row.get(2).unwrap(),
        ))
    }) else {
        return Vec::new();
    };

    query_map.map(|r| r.unwrap()).collect::<Vec<_>>()
}

pub fn get_formats(conn: &Connection, gen: Generation) -> Vec<String> {
    let tbl = get_table_name(gen);
    let mut stmt = conn
        .prepare(&format!(
            "SELECT main_format from {tbl} group by main_format"
        ))
        .expect("Failed to prepare statement");
    stmt.query_map([], |row| Ok(row.get(0).unwrap()))
        .expect("Failed to execute get formats")
        .map(|r| r.unwrap())
        .collect::<Vec<_>>()
}

pub fn get_pokemon(id: usize, conn: &Connection, gen: Generation) -> Pokemon {
    let tbl = get_table_name(gen);
    let mut stmt = conn
        .prepare(&format!("SELECT * from {tbl} where id = ?"))
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
            tera_type: row.get(11).unwrap(),
            complete: row.get(12).unwrap(),
        })
    })
    .expect("Failed to query statement")
}

fn get_table_name(gen: Generation) -> &'static str {
    match gen {
        Generation::ScarletViolet => "gen9",
        Generation::SwordShield => "gen8",
        Generation::SunMoon => "gen7",
        Generation::XY => "gen6",
        Generation::BlackWhite => "gen5",
        Generation::DiamondPearl => "gen4",
        Generation::RubySapphire => "gen3",
        Generation::GoldSilver => "gen2",
        Generation::RedBlue => "gen1",
    }
}

pub fn set_complete(id: usize, conn: &Connection, gen: Generation) {
    let tbl = get_table_name(gen);
    let _ = conn.execute(
        &format!("UPDATE {tbl} SET complete = true WHERE id = ?"),
        [id],
    );
}

pub fn set_incomplete(id: usize, conn: &Connection, gen: Generation) {
    let tbl = get_table_name(gen);
    let _ = conn.execute(
        &format!("UPDATE {tbl} SET complete = false WHERE id = ?"),
        [id],
    );
}

pub fn table_exists(conn: &Connection, gen: Generation) -> bool {
    let tbl = get_table_name(gen);
    conn.prepare(&format!("SELECT COUNT(*) from {tbl}")).is_ok()
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Format {
    None,
    Uber,
    NFE,
    LC,
    NUBL,
    RUBL,
    UUBL,
    PU,
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
            "PU" => Format::PU,
            "Uber" => Format::Uber,
            "UUBL" => Format::UUBL,
            "NUBL" => Format::NUBL,
            "RUBL" => Format::RUBL,
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
            Format::PU => write!(f, "PU"),
            Format::RUBL => write!(f, "RUBL"),
            Format::UUBL => write!(f, "UUBL"),
            Format::NUBL => write!(f, "NUBL"),
            Format::NFE => write!(f, "NFE"),
            Format::LC => write!(f, "LC"),
        }
    }
}
