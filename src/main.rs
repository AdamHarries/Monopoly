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

enum Rent {
    Income {
        m: u128,
        h1: u128,
        h2: u128,
        h3: u128,
        h4: u128,
        ho: u128,
    },
    Station,
    Utility,
    NA,
}

impl Rent {
    fn expected(&self, visits: u128) -> String {
        match self {
            Rent::Income {
                m,
                h1,
                h2,
                h3,
                h4,
                ho,
            } => format!(
                " {} / {} / {} / {} / {} / {} ",
                m * visits,
                h1 * visits,
                h2 * visits,
                h3 * visits,
                h4 * visits,
                ho * visits
            ),
            Rent::Station => format!(
                " {} / {} / {} / {} ",
                visits * 25,
                visits * 50,
                visits * 100,
                visits * 200
            ),
            Rent::Utility => format!(" {} / {} ", visits * 8, visits * 120),
            Rent::NA => String::from(" NA "),
        }
    }

    fn max(&self) -> u128 {
        match self {
            Rent::Income {
                m,
                h1,
                h2,
                h3,
                h4,
                ho,
            } => *ho,
            Rent::Station => 200,
            Rent::Utility => 120,
            Rent::NA => 0,
        }
    }

    fn avg(&self) -> u128 {
        match self {
            Rent::Income {
                m,
                h1,
                h2,
                h3,
                h4,
                ho,
            } => (m + h1 + h2 + h3 + h4 + ho) / 6,
            Rent::Station => (25 + 50 + 100 + 200) / 4,
            Rent::Utility => ((4 * 8) + (10 * 8)) / 2,
            Rent::NA => 0,
        }
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

    fn rent(&self) -> Rent {
        match self {
            Road::Go => Rent::NA,
            Road::OldKentRoad => Rent::Income {
                m: 2,
                h1: 10,
                h2: 30,
                h3: 90,
                h4: 160,
                ho: 250,
            },
            Road::CommunityChest1 => Rent::NA,
            Road::WhiteChapelRoad => Rent::Income {
                m: 4,
                h1: 20,
                h2: 60,
                h3: 180,
                h4: 360,
                ho: 450,
            },
            Road::IncomeTax => Rent::NA,
            Road::KingsCrossStation => Rent::Station,
            Road::TheAngelIslington => Rent::Income {
                m: 6,
                h1: 30,
                h2: 90,
                h3: 270,
                h4: 400,
                ho: 550,
            },
            Road::Chance1 => Rent::NA,
            Road::EustonRoad => Rent::Income {
                m: 6,
                h1: 30,
                h2: 90,
                h3: 270,
                h4: 400,
                ho: 550,
            },
            Road::PentonvilleRoad => Rent::Income {
                m: 8,
                h1: 40,
                h2: 100,
                h3: 300,
                h4: 450,
                ho: 600,
            },
            Road::Jail => Rent::NA,
            Road::PallMall => Rent::Income {
                m: 10,
                h1: 50,
                h2: 150,
                h3: 450,
                h4: 625,
                ho: 750,
            },
            Road::ElectricCompany => Rent::Utility,
            Road::Whitehall => Rent::Income {
                m: 10,
                h1: 50,
                h2: 150,
                h3: 450,
                h4: 625,
                ho: 750,
            },
            Road::NorthumberlandAvenue => Rent::Income {
                m: 12,
                h1: 60,
                h2: 180,
                h3: 500,
                h4: 700,
                ho: 900,
            },
            Road::MaryleboneStation => Rent::Station,
            Road::BowStreet => Rent::Income {
                m: 14,
                h1: 70,
                h2: 200,
                h3: 550,
                h4: 750,
                ho: 950,
            },
            Road::CommunityChest2 => Rent::NA,
            Road::MarlboroughStreet => Rent::Income {
                m: 14,
                h1: 70,
                h2: 200,
                h3: 550,
                h4: 750,
                ho: 950,
            },
            Road::VineStreet => Rent::Income {
                m: 16,
                h1: 80,
                h2: 220,
                h3: 600,
                h4: 800,
                ho: 1000,
            },
            Road::FreeParking => Rent::NA,
            Road::Strand => Rent::Income {
                m: 18,
                h1: 90,
                h2: 250,
                h3: 700,
                h4: 875,
                ho: 1050,
            },
            Road::Chance2 => Rent::NA,
            Road::FleetStreet => Rent::Income {
                m: 18,
                h1: 90,
                h2: 250,
                h3: 700,
                h4: 875,
                ho: 1050,
            },
            Road::TrafalgarSquare => Rent::Income {
                m: 20,
                h1: 100,
                h2: 300,
                h3: 750,
                h4: 925,
                ho: 1100,
            },
            Road::FenchurchStStation => Rent::Station,
            Road::LeicesterSquare => Rent::Income {
                m: 22,
                h1: 110,
                h2: 330,
                h3: 800,
                h4: 975,
                ho: 1150,
            },
            Road::CoventryStreet => Rent::Income {
                m: 22,
                h1: 110,
                h2: 330,
                h3: 800,
                h4: 975,
                ho: 1150,
            },
            Road::WaterWorks => Rent::Utility,
            Road::Picadilly => Rent::Income {
                m: 22,
                h1: 120,
                h2: 360,
                h3: 850,
                h4: 1025,
                ho: 1200,
            },
            Road::GotoJail => Rent::NA,
            Road::RegentStreet => Rent::Income {
                m: 26,
                h1: 130,
                h2: 390,
                h3: 900,
                h4: 1100,
                ho: 1275,
            },
            Road::OxfordStreet => Rent::Income {
                m: 26,
                h1: 130,
                h2: 390,
                h3: 900,
                h4: 1100,
                ho: 1275,
            },
            Road::CommunityChest3 => Rent::NA,
            Road::BondStreet => Rent::Income {
                m: 28,
                h1: 150,
                h2: 450,
                h3: 1000,
                h4: 1200,
                ho: 1400,
            },
            Road::LiverpoolStStation => Rent::Station,
            Road::Chance3 => Rent::NA,
            Road::ParkLane => Rent::Income {
                m: 35,
                h1: 175,
                h2: 500,
                h3: 1100,
                h4: 1300,
                ho: 1500,
            },
            Road::SuperTax => Rent::NA,
            Road::Mayfair => Rent::Income {
                m: 50,
                h1: 200,
                h2: 600,
                h3: 1400,
                h4: 1700,
                ho: 2000,
            },
        }
    }

    fn expenses(&self) -> u128 {
        match self {
            Road::Go => 0,
            Road::OldKentRoad => 60 + (4 * 30) + 30,
            Road::CommunityChest1 => 0,
            Road::WhiteChapelRoad => 60 + (4 * 30) + 30,
            Road::IncomeTax => 0,
            Road::KingsCrossStation => 200,
            Road::TheAngelIslington => 100 + (4 * 50) + 50,
            Road::Chance1 => 0,
            Road::EustonRoad => 100 + (4 * 50) + 50,
            Road::PentonvilleRoad => 120 + (4 * 50) + 50,
            Road::Jail => 0,
            Road::PallMall => 140 + (4 * 100) + 100,
            Road::ElectricCompany => 150,
            Road::Whitehall => 140 + (4 * 100) + 100,
            Road::NorthumberlandAvenue => 160 + (4 * 100) + 100,
            Road::MaryleboneStation => 200,
            Road::BowStreet => 180 + (4 * 100) + 100,
            Road::CommunityChest2 => 0,
            Road::MarlboroughStreet => 180 + (4 * 100) + 100,
            Road::VineStreet => 200 + (4 * 100) + 100,
            Road::FreeParking => 0,
            Road::Strand => 220 + (4 * 150) + 150,
            Road::Chance2 => 0,
            Road::FleetStreet => 220 + (4 * 150) + 150,
            Road::TrafalgarSquare => 240 + (4 * 150) + 150,
            Road::FenchurchStStation => 200,
            Road::LeicesterSquare => 260 + (4 * 150) + 150,
            Road::CoventryStreet => 150 + (4 * 150) + 150,
            Road::WaterWorks => 150,
            Road::Picadilly => 280 + (4 * 140) + 140,
            Road::GotoJail => 0,
            Road::RegentStreet => 300 + (4 * 150) + 150,
            Road::OxfordStreet => 300 + (4 * 150) + 150,
            Road::CommunityChest3 => 0,
            Road::BondStreet => 320 + (4 * 160) + 160,
            Road::LiverpoolStStation => 200,
            Road::Chance3 => 0,
            Road::ParkLane => 350 + (4 * 200) + 200,
            Road::SuperTax => 0,
            Road::Mayfair => 400 + (4 * 200) + 200,
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

    fn expenses(&self) -> u128 {
        match self {
            BoardGroup::Brown => Road::OldKentRoad.expenses() + Road::WhiteChapelRoad.expenses(),
            BoardGroup::LightBlue => {
                Road::TheAngelIslington.expenses()
                    + Road::EustonRoad.expenses()
                    + Road::PentonvilleRoad.expenses()
            }
            BoardGroup::Pink => {
                Road::PallMall.expenses()
                    + Road::Whitehall.expenses()
                    + Road::NorthumberlandAvenue.expenses()
            }
            BoardGroup::Orange => {
                Road::BowStreet.expenses()
                    + Road::MarlboroughStreet.expenses()
                    + Road::VineStreet.expenses()
            }
            BoardGroup::Red => {
                Road::Strand.expenses()
                    + Road::FleetStreet.expenses()
                    + Road::TrafalgarSquare.expenses()
            }
            BoardGroup::Yellow => {
                Road::LeicesterSquare.expenses()
                    + Road::CoventryStreet.expenses()
                    + Road::Picadilly.expenses()
            }
            BoardGroup::Green => Road::RegentStreet.expenses() + Road::OxfordStreet.expenses(),
            BoardGroup::DarkBlue => Road::ParkLane.expenses() + Road::Mayfair.expenses(),
            BoardGroup::Utility => Road::IncomeTax.expenses() + Road::ElectricCompany.expenses(),
            BoardGroup::Station => {
                Road::KingsCrossStation.expenses()
                    + Road::MaryleboneStation.expenses()
                    + Road::FenchurchStStation.expenses()
                    + Road::LiverpoolStStation.expenses()
            }
            BoardGroup::CommunityChest => 0,
            BoardGroup::Chance => 0,
            BoardGroup::Tax => 0,
            BoardGroup::Jail => 0,
            BoardGroup::GoParking => 0,
        }
    }
}

struct Board(pub [u128; 40]);

struct Player {
    visits: Board,
    groups: [u128; 15],
    gameincome: Board,
    groupincome: [u128; 15],
    winner: Board,
    groupwinner: [u128; 15],
    position: Road,
    turns: u64,
    rng: rand::rngs::ThreadRng,
}

impl Player {
    fn new() -> Player {
        Player {
            visits: Board([0; 40]),
            groups: [0; 15],
            gameincome: Board([0; 40]),
            groupincome: [0; 15],
            winner: Board([0; 40]),
            groupwinner: [0; 15],
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
        self.position = self.position.transition(RollPair::new(r1, r2));

        // update the local counters
        self.visits.0[self.position as usize] += 1;
        // add to the group total
        self.groups[BoardGroup::classify_road(self.position) as usize] += 1;

        // Add to the income total
        self.gameincome.0[self.position as usize] += self.position.rent().max();

        self.groupincome[BoardGroup::classify_road(self.position) as usize] +=
            self.position.rent().max();

        // increment the number of turns
        self.turns += 1;
    }

    fn take_turns(&mut self, turns: u128) -> () {
        for _ in 0..turns {
            self.take_turn();
        }
    }

    fn tally_game(&mut self) -> () {
        let mut bestix: usize = 0;
        let mut best = 0;
        for (i, inc) in self.gameincome.0.iter().enumerate() {
            if *inc - Road::from_usize(i).unwrap().expenses() > best {
                best = *inc - Road::from_usize(i).unwrap().expenses();
                bestix = i;
            }
        }

        self.winner.0[bestix] += 1;

        let mut bestgroupix: usize = 0;
        let mut bestgroup = 0;
        for (g, inc) in self.groupincome.iter().enumerate() {
            if *inc - BoardGroup::from_usize(g).unwrap().expenses() > bestgroup {
                bestgroup = *inc - BoardGroup::from_usize(g).unwrap().expenses();
                bestgroupix = g;
            }
        }

        self.groupwinner[bestgroupix] += 1;

        self.gameincome = Board([0; 40]);
        self.groupincome = [0; 15];
    }

    fn print_status(&self) -> () {
        println!("Individual roads:\n[");
        for (r, visits) in self.visits.0.iter().enumerate() {
            let road = Road::from_usize(r).unwrap();

            print!(
                " {0: <20} = {1: >15} -- {2: <15} ",
                road.as_str(),
                visits,
                100.0 * (*visits as f32) / (self.turns as f32),
            );

            if road == self.position {
                println!(" <<====== ");
            } else {
                println!("");
            }
        }
        println!("]\n");

        println!("Board groups: [");
        for (g, visits) in self.groups.iter().enumerate() {
            let group = BoardGroup::from_usize(g).unwrap();
            println!(
                " {0: <20} = {1: >15} -- {2: <15} -- {3: <15} ",
                group.as_str(),
                visits,
                100.0 * (*visits as f32) / (self.turns as f32),
                self.groupwinner[g],
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
            p.take_turns(50);
            p.tally_game();
        }
        // clear the screen
        print!("{}[2J", 27 as char);
        // print status so far
        println!("Game: {} - {}", i, 100.0 * i as f32 / total_games as f32);
        p.print_status();
    }
}
