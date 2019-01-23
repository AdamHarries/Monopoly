#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
extern crate rand;

use num_traits::FromPrimitive;
use rand::prelude::*;

#[derive(Primitive)]
enum DieRoll {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl DieRoll {
    fn roll<T: Rng>(rng: &mut T) -> DieRoll {
        let r: u8 = rng.gen_range(1, 7);
        DieRoll::from_u8(r).unwrap()
    }
}

struct RollPair {
    r1: DieRoll,
    r2: DieRoll,
}

impl RollPair {
    fn new(r1: DieRoll, r2: DieRoll) -> RollPair {
        RollPair { r1: r1, r2: r2 }
    }
    fn eval(self) -> u8 {
        self.r1 as u8 + self.r2 as u8
    }
}

#[derive(Primitive, Clone, Copy, PartialEq, Eq, Debug)]
enum Road {
    Go = 0,
    OldKentRoad = 1,
    CommunityChest1 = 2,
    WhiteChapelRoad = 3,
    IncomeTax = 4,
    KingsCrossStation = 5,
    TheAngelIslington = 6,
    Chance1 = 7,
    EustonRoad = 8,
    PentonvilleRoad = 9,
    Jail = 10,
    PallMall = 11,
    ElectricCompany = 12,
    Whitehall = 13,
    NorthumberlandAvenue = 14,
    MaryleboneStation = 15,
    BowStreet = 16,
    CommunityChest2 = 17,
    MarlboroughStreet = 18,
    VineStreet = 19,
    FreeParking = 20,
    Strand = 21,
    Chance2 = 22,
    FleetStreet = 23,
    TrafalgarSquare = 24,
    FenchurchStStation = 25,
    LeicesterSquare = 26,
    CoventryStreet = 27,
    WaterWorks = 28,
    Picadilly = 29,
    GotoJail = 30,
    RegentStreet = 31,
    OxfordStreet = 32,
    CommunityChest3 = 33,
    BondStreet = 34,
    LiverpoolStStation = 35,
    Chance3 = 36,
    ParkLane = 37,
    SuperTax = 38,
    Mayfair = 39,
}

impl Road {
    fn transition(self, r: RollPair) -> Road {
        // check to see if we've landed on "GoToJail", in which case, transition to Jail, but don't record extra for jail
        let pos = match self {
            Road::GotoJail => Road::Jail,
            r => r,
        };
        // Get the self as a u8.
        let u = pos as u8;
        // Add on the roll, and wrap around.
        Road::from_u8((u + r.eval()) % (Road::Mayfair as u8 + 1)).unwrap()
    }

    fn as_str(&self) -> &'static str {
        match self {
            Road::Go => "Go",
            Road::OldKentRoad => "OldKentRoad",
            Road::CommunityChest1 => "CommunityChest1",
            Road::WhiteChapelRoad => "WhiteChapelRoad",
            Road::IncomeTax => "IncomeTax",
            Road::KingsCrossStation => "KingsCrossStation",
            Road::TheAngelIslington => "TheAngelIslington",
            Road::Chance1 => "Chance1",
            Road::EustonRoad => "EustonRoad",
            Road::PentonvilleRoad => "PentonvilleRoad",
            Road::Jail => "Jail",
            Road::PallMall => "PallMall",
            Road::ElectricCompany => "ElectricCompany",
            Road::Whitehall => "Whitehall",
            Road::NorthumberlandAvenue => "NorthumberlandAvenue",
            Road::MaryleboneStation => "MaryleboneStation",
            Road::BowStreet => "BowStreet",
            Road::CommunityChest2 => "CommunityChest2",
            Road::MarlboroughStreet => "MarlboroughStreet",
            Road::VineStreet => "VineStreet",
            Road::FreeParking => "FreeParking",
            Road::Strand => "Strand",
            Road::Chance2 => "Chance2",
            Road::FleetStreet => "FleetStreet",
            Road::TrafalgarSquare => "TrafalgarSquare",
            Road::FenchurchStStation => "FenchurchStStation",
            Road::LeicesterSquare => "LeicesterSquare",
            Road::CoventryStreet => "CoventryStreet",
            Road::WaterWorks => "WaterWorks",
            Road::Picadilly => "Picadilly",
            Road::GotoJail => "GotoJail",
            Road::RegentStreet => "RegentStreet",
            Road::OxfordStreet => "OxfordStreet",
            Road::CommunityChest3 => "CommunityChest3",
            Road::BondStreet => "BondStreet",
            Road::LiverpoolStStation => "LiverpoolStStation",
            Road::Chance3 => "Chance3",
            Road::ParkLane => "ParkLane",
            Road::SuperTax => "SuperTax",
            Road::Mayfair => "Mayfair",
        }
    }
}

#[derive(Primitive, Clone, Copy, PartialEq, Eq, Debug)]
enum BoardGroup {
    Brown = 0,
    LightBlue = 1,
    Pink = 2,
    Orange = 3,
    Red = 4,
    Yellow = 5,
    Green = 6,
    DarkBlue = 7,
    Utility = 8,
    Station = 9,
    CommunityChest = 10,
    Chance = 11,
    Tax = 12,
    Jail = 13,
    GoParking = 14,
}

impl BoardGroup {
    fn classify_road(r: Road) -> BoardGroup {
        match r {
            Road::Go => BoardGroup::GoParking,
            Road::OldKentRoad => BoardGroup::Brown,
            Road::CommunityChest1 => BoardGroup::CommunityChest,
            Road::WhiteChapelRoad => BoardGroup::Brown,
            Road::IncomeTax => BoardGroup::Tax,
            Road::KingsCrossStation => BoardGroup::Station,
            Road::TheAngelIslington => BoardGroup::LightBlue,
            Road::Chance1 => BoardGroup::Chance,
            Road::EustonRoad => BoardGroup::LightBlue,
            Road::PentonvilleRoad => BoardGroup::LightBlue,
            Road::Jail => BoardGroup::Jail,
            Road::PallMall => BoardGroup::Pink,
            Road::ElectricCompany => BoardGroup::Utility,
            Road::Whitehall => BoardGroup::Pink,
            Road::NorthumberlandAvenue => BoardGroup::Pink,
            Road::MaryleboneStation => BoardGroup::Station,
            Road::BowStreet => BoardGroup::Orange,
            Road::CommunityChest2 => BoardGroup::CommunityChest,
            Road::MarlboroughStreet => BoardGroup::Orange,
            Road::VineStreet => BoardGroup::Orange,
            Road::FreeParking => BoardGroup::GoParking,
            Road::Strand => BoardGroup::Red,
            Road::Chance2 => BoardGroup::Chance,
            Road::FleetStreet => BoardGroup::Red,
            Road::TrafalgarSquare => BoardGroup::Red,
            Road::FenchurchStStation => BoardGroup::Station,
            Road::LeicesterSquare => BoardGroup::Yellow,
            Road::CoventryStreet => BoardGroup::Yellow,
            Road::WaterWorks => BoardGroup::Utility,
            Road::Picadilly => BoardGroup::Yellow,
            Road::GotoJail => BoardGroup::Jail,
            Road::RegentStreet => BoardGroup::Green,
            Road::OxfordStreet => BoardGroup::Green,
            Road::CommunityChest3 => BoardGroup::CommunityChest,
            Road::BondStreet => BoardGroup::Green,
            Road::LiverpoolStStation => BoardGroup::Station,
            Road::Chance3 => BoardGroup::Chance,
            Road::ParkLane => BoardGroup::DarkBlue,
            Road::SuperTax => BoardGroup::Tax,
            Road::Mayfair => BoardGroup::DarkBlue,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            BoardGroup::Brown => "Brown",
            BoardGroup::LightBlue => "LightBlue",
            BoardGroup::Pink => "Pink",
            BoardGroup::Orange => "Orange",
            BoardGroup::Red => "Red",
            BoardGroup::Yellow => "Yellow",
            BoardGroup::Green => "Green",
            BoardGroup::DarkBlue => "DarkBlue",
            BoardGroup::Utility => "Utility",
            BoardGroup::Station => "Station",
            BoardGroup::CommunityChest => "CommunityChest",
            BoardGroup::Chance => "Chance",
            BoardGroup::Tax => "Tax",
            BoardGroup::Jail => "Jail",
            BoardGroup::GoParking => "GoParking",
        }
    }
}

struct Board(pub [u32; 40]);

struct Player {
    visits: Board,
    position: Road,
    turns: u64,
    rng: rand::rngs::ThreadRng,
}

impl Player {
    fn new() -> Player {
        Player {
            visits: Board([0; 40]),
            position: Road::Go,
            turns: 0,
            rng: thread_rng(),
        }
    }

    fn take_turn(&mut self) -> () {
        // roll the dice
        let r1 = DieRoll::roll(&mut self.rng);
        let r2 = DieRoll::roll(&mut self.rng);

        // move
        self.position = self.position.clone().transition(RollPair::new(r1, r2));

        // update the local counters
        self.visits.0[self.position.clone() as usize] += 1;

        // increment the number of turns
        self.turns += 1;
    }

    fn take_turns(&mut self, turns: u32) -> () {
        for _ in 0..turns {
            self.take_turn();
        }
    }

    fn print_status(&self) -> () {
        let mut groups = [0 as u32; 15];
        println!("Individual roads:\n[");
        for (r, visits) in self.visits.0.iter().enumerate() {
            let road = Road::from_usize(r).unwrap();
            print!(
                " {0: <20} = {1: >15} -- {2: <15}",
                road.as_str(),
                visits,
                100.0 * (*visits as f32) / (self.turns as f32)
            );

            if road == self.position {
                println!(" <<====== ");
            } else {
                println!("");
            }

            // add to the group total
            groups[BoardGroup::classify_road(road) as usize] += visits;
        }
        println!("]\n");

        println!("Board groups: [");
        for (g, visits) in groups.iter().enumerate() {
            let group = BoardGroup::from_usize(g).unwrap();
            println!(
                " {0: <20} = {1: >15} -- {2: <15}",
                group.as_str(),
                visits,
                100.0 * (*visits as f32) / (self.turns as f32)
            );
        }
        println!("]");
    }
}

fn main() {
    let mut p = Player::new();
    p.visits.0[0] = 1;
    let total_games = 1000000000;
    for i in 0..total_games {
        // run 1000 games
        for _ in 0..1000 {
            // reset the player to the start of a game
            p.position = Road::Go;
            p.take_turns(100);
        }
        // clear the screen
        print!("{}[2J", 27 as char);
        // print status so far
        println!("Game: {} - {}", i, 100.0 * i as f32 / total_games as f32);
        p.print_status();
    }
}
