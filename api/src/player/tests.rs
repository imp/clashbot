use serde_json as json;

use super::*;

const PLAYER_JSON: &str = r##"
{
    "achievements": [
        {
            "completionInfo": "Highest Gold Storage level: 14",
            "info": "Upgrade a Gold Storage to level 10",
            "name": "Bigger Coffers",
            "stars": 3,
            "target": 10,
            "value": 14,
            "village": "home"
        },
        {
            "completionInfo": "Stars in Campaign Map: 168",
            "info": "Win 150 Stars on the Campaign Map",
            "name": "Get those Goblins!",
            "stars": 3,
            "target": 150,
            "value": 168,
            "village": "home"
        },
        {
            "completionInfo": "Current Town Hall level: 13",
            "info": "Upgrade Town Hall to level 8",
            "name": "Bigger & Better",
            "stars": 3,
            "target": 8,
            "value": 13,
            "village": "home"
        },
        {
            "completionInfo": "Total obstacles removed: 5370",
            "info": "Remove 500 obstacles (trees, rocks, bushes)",
            "name": "Nice and Tidy",
            "stars": 3,
            "target": 500,
            "value": 5370,
            "village": "home"
        },
        {
            "completionInfo": null,
            "info": "Unlock Dragon in the Barracks",
            "name": "Discover New Troops",
            "stars": 3,
            "target": 1,
            "value": 1,
            "village": "home"
        },
        {
            "completionInfo": "Total Gold looted: 1423282022",
            "info": "Steal 100000000 gold",
            "name": "Gold Grab",
            "stars": 3,
            "target": 100000000,
            "value": 1423282022,
            "village": "home"
        },
        {
            "completionInfo": "Total Elixir looted: 1444195041",
            "info": "Steal 100000000 elixir",
            "name": "Elixir Escapade",
            "stars": 3,
            "target": 100000000,
            "value": 1444195041,
            "village": "home"
        },
        {
            "completionInfo": "Trophy record: 5169",
            "info": "Achieve a total of 1250 trophies in Multiplayer battles",
            "name": "Sweet Victory!",
            "stars": 3,
            "target": 1250,
            "value": 5169,
            "village": "home"
        },
        {
            "completionInfo": "Current Clan Castle level: 9",
            "info": "Upgrade Clan Castle to level 4",
            "name": "Empire Builder",
            "stars": 3,
            "target": 4,
            "value": 9,
            "village": "home"
        },
        {
            "completionInfo": "Total walls destroyed: 87184",
            "info": "Destroy 2000 Walls in Multiplayer battles",
            "name": "Wall Buster",
            "stars": 3,
            "target": 2000,
            "value": 87184,
            "village": "home"
        },
        {
            "completionInfo": "Total Town Halls destroyed: 3702",
            "info": "Destroy 2000 Town Halls in Multiplayer battles",
            "name": "Humiliator",
            "stars": 3,
            "target": 2000,
            "value": 3702,
            "village": "home"
        },
        {
            "completionInfo": "Total Builder's Huts destroyed: 16745",
            "info": "Destroy 2500 Builder's Huts in Multiplayer battles",
            "name": "Union Buster",
            "stars": 3,
            "target": 2500,
            "value": 16745,
            "village": "home"
        },
        {
            "completionInfo": "Total multiplayer battles won: 4860",
            "info": "Win 5000 Multiplayer battles",
            "name": "Conqueror",
            "stars": 2,
            "target": 5000,
            "value": 4860,
            "village": "home"
        },
        {
            "completionInfo": "Total defenses won: 1288",
            "info": "Successfully defend against 5000 attacks",
            "name": "Unbreakable",
            "stars": 2,
            "target": 5000,
            "value": 1288,
            "village": "home"
        },
        {
            "completionInfo": "Total capacity donated: 148905",
            "info": "Donate 25000 capacity worth of reinforcements to Clanmates",
            "name": "Friend in Need",
            "stars": 3,
            "target": 25000,
            "value": 148905,
            "village": "home"
        },
        {
            "completionInfo": "Total Mortars destroyed: 11347",
            "info": "Destroy 5000 Mortars in Multiplayer battles",
            "name": "Mortar Mauler",
            "stars": 3,
            "target": 5000,
            "value": 11347,
            "village": "home"
        },
        {
            "completionInfo": "Total Dark Elixir looted: 11613102",
            "info": "Steal 1000000 Dark Elixir",
            "name": "Heroic Heist",
            "stars": 3,
            "target": 1000000,
            "value": 11613102,
            "village": "home"
        },
        {
            "completionInfo": null,
            "info": "Become a Champion!",
            "name": "League All-Star",
            "stars": 3,
            "target": 1,
            "value": 22,
            "village": "home"
        },
        {
            "completionInfo": "Total X-Bows destroyed: 6676",
            "info": "Destroy 2500 X-Bows in Multiplayer battles",
            "name": "X-Bow Exterminator",
            "stars": 3,
            "target": 2500,
            "value": 6676,
            "village": "home"
        },
        {
            "completionInfo": "Total Inferno Towers destroyed: 3778",
            "info": "Destroy 5000 Inferno Towers in Multiplayer battles",
            "name": "Firefighter",
            "stars": 2,
            "target": 5000,
            "value": 3778,
            "village": "home"
        },
        {
            "completionInfo": "Total Stars scored for clan in Clan War battles: 652",
            "info": "Score 1000 Stars for your clan in Clan War battles",
            "name": "War Hero",
            "stars": 2,
            "target": 1000,
            "value": 652,
            "village": "home"
        },
        {
            "completionInfo": "Total gold collected in Clan War bonuses: 868697232",
            "info": "Collect 100000000 gold from the Clan Castle",
            "name": "Clan War Wealth",
            "stars": 3,
            "target": 100000000,
            "value": 868697232,
            "village": "home"
        },
        {
            "completionInfo": "Total Eagle Artilleries destroyed: 1635",
            "info": "Destroy 2000 Eagle Artilleries in Multiplayer battles",
            "name": "Anti-Artillery",
            "stars": 2,
            "target": 2000,
            "value": 1635,
            "village": "home"
        },
        {
            "completionInfo": "Total spell capacity donated: 5035",
            "info": "Donate 10000 spell storage capacity worth of spells",
            "name": "Sharing is caring",
            "stars": 2,
            "target": 10000,
            "value": 5035,
            "village": "home"
        },
        {
            "completionInfo": "Completed!",
            "info": "Protect your village by connecting to a social network",
            "name": "Keep Your Account Safe!",
            "stars": 0,
            "target": 1,
            "value": 0,
            "village": "home"
        },
        {
            "completionInfo": "Current Builder Hall level: 9",
            "info": "Upgrade Builder Hall to level 8",
            "name": "Master Engineering",
            "stars": 3,
            "target": 8,
            "value": 9,
            "village": "builderBase"
        },
        {
            "completionInfo": null,
            "info": "Unlock Super P.E.K.K.A in the Builder Barracks",
            "name": "Next Generation Model",
            "stars": 3,
            "target": 1,
            "value": 1,
            "village": "builderBase"
        },
        {
            "completionInfo": "Total Builder Halls destroyed: 4247",
            "info": "Destroy 2000 Builder Halls in Versus battles",
            "name": "Un-Build It",
            "stars": 3,
            "target": 2000,
            "value": 4247,
            "village": "builderBase"
        },
        {
            "completionInfo": "Versus Trophy record: 5102",
            "info": "Achieve a total of 3000 trophies in Versus battles",
            "name": "Champion Builder",
            "stars": 3,
            "target": 3000,
            "value": 5102,
            "village": "builderBase"
        },
        {
            "completionInfo": "Total buildings geared up: 3",
            "info": "Gear Up 3 buildings using the Master Builder",
            "name": "High Gear",
            "stars": 3,
            "target": 3,
            "value": 3,
            "village": "builderBase"
        },
        {
            "completionInfo": null,
            "info": "Rebuild Battle Machine",
            "name": "Hidden Treasures",
            "stars": 3,
            "target": 1,
            "value": 1,
            "village": "builderBase"
        },
        {
            "completionInfo": "Total Clan Games points: 129950",
            "info": "Earn 100000 points in Clan Games",
            "name": "Games Champion",
            "stars": 3,
            "target": 100000,
            "value": 129950,
            "village": "home"
        },
        {
            "completionInfo": null,
            "info": "Slay the Giant Dragon",
            "name": "Dragon Slayer",
            "stars": 1,
            "target": 1,
            "value": 1,
            "village": "home"
        },
        {
            "completionInfo": "Total Stars scored for clan in War League battles: 140",
            "info": "Score 250 Stars for your clan in War League battles",
            "name": "War League Legend",
            "stars": 2,
            "target": 250,
            "value": 140,
            "village": "home"
        },
        {
            "completionInfo": "Completed!",
            "info": "Connect your account to Supercell ID for safe keeping.",
            "name": "Keep Your Account Safe!",
            "stars": 0,
            "target": 1,
            "value": 0,
            "village": "home"
        },
        {
            "completionInfo": "Total Season Challenges points: 122520",
            "info": "Earn 50000 points in Season Challenges",
            "name": "Well Seasoned",
            "stars": 3,
            "target": 50000,
            "value": 122520,
            "village": "home"
        },
        {
            "completionInfo": "Total Scattershots destroyed: 399",
            "info": "Destroy 400 Scattershots in Multiplayer battles",
            "name": "Shattered and Scattered",
            "stars": 1,
            "target": 400,
            "value": 399,
            "village": "home"
        },
        {
            "completionInfo": "Weaponized Town Halls destroyed: 1095",
            "info": "Destroy 2000 weaponized Town Halls in Multiplayer battles",
            "name": "Not So Easy This Time",
            "stars": 2,
            "target": 2000,
            "value": 1095,
            "village": "home"
        },
        {
            "completionInfo": "Total weaponized Builder's Huts destroyed: 222",
            "info": "Destroy 250 weaponized Builder's Huts in Multiplayer battles",
            "name": "Bust This!",
            "stars": 1,
            "target": 250,
            "value": 222,
            "village": "home"
        },
        {
            "completionInfo": "Total times Super Troops boosted: 17",
            "info": "Boost a Super Troop 20 times",
            "name": "Superb Work",
            "stars": 0,
            "target": 20,
            "value": 17,
            "village": "home"
        },
        {
            "completionInfo": "Total Siege Machines donated: 922",
            "info": "Donate 1000 Siege Machines",
            "name": "Siege Sharer",
            "stars": 1,
            "target": 1000,
            "value": 922,
            "village": "home"
        },
        {
            "completionInfo": "Total Capital Gold looted: 63815",
            "info": "Loot 250000 Capital Gold during Raid attacks",
            "name": "Aggressive Capitalism",
            "stars": 1,
            "target": 250000,
            "value": 63815,
            "village": "home"
        },
        {
            "completionInfo": "Total Capital Gold contributed: 98168",
            "info": "Contribute 500000 Capital Gold to upgrades in the Clan Capital",
            "name": "Most Valuable Clanmate",
            "stars": 1,
            "target": 500000,
            "value": 98168,
            "village": "home"
        }
    ],
    "attackWins": 87,
    "bestTrophies": 5169,
    "bestVersusTrophies": 5102,
    "builderHallLevel": 9,
    "clan": {
        "badgeUrls": {
            "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
            "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
            "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "name": "Best Clan Ever",
        "tag": "#202QTU40"
    },
    "defenseWins": 1,
    "donations": 6984,
    "donationsReceived": 7358,
    "expLevel": 210,
    "heroes": [
        {
            "level": 67,
            "maxLevel": 80,
            "name": "Barbarian King",
            "village": "home"
        },
        {
            "level": 75,
            "maxLevel": 80,
            "name": "Archer Queen",
            "village": "home"
        },
        {
            "level": 44,
            "maxLevel": 55,
            "name": "Grand Warden",
            "village": "home"
        },
        {
            "level": 30,
            "maxLevel": 30,
            "name": "Battle Machine",
            "village": "builderBase"
        },
        {
            "level": 11,
            "maxLevel": 30,
            "name": "Royal Champion",
            "village": "home"
        }
    ],
    "labels": [
        {
            "iconUrls": {
                "medium": "https://api-assets.clashofclans.com/labels/128/MvL0LDt0yv9AI-Vevpu8yE5NAJUIV05Ofpsr4IfGRxQ.png",
                "small": "https://api-assets.clashofclans.com/labels/64/MvL0LDt0yv9AI-Vevpu8yE5NAJUIV05Ofpsr4IfGRxQ.png"
            },
            "id": 57000008,
            "name": "Active Donator"
        },
        {
            "iconUrls": {
                "medium": "https://api-assets.clashofclans.com/labels/128/mcWhk0ii7CyjiiHOidhRofrSulpVrxjDu24cQtGCQbE.png",
                "small": "https://api-assets.clashofclans.com/labels/64/mcWhk0ii7CyjiiHOidhRofrSulpVrxjDu24cQtGCQbE.png"
            },
            "id": 57000009,
            "name": "Active Daily"
        },
        {
            "iconUrls": {
                "medium": "https://api-assets.clashofclans.com/labels/128/t0KZ4173i9vJFrD5F06-2TFNFk9UwJXxPjfutcG-dig.png",
                "small": "https://api-assets.clashofclans.com/labels/64/t0KZ4173i9vJFrD5F06-2TFNFk9UwJXxPjfutcG-dig.png"
            },
            "id": 57000011,
            "name": "Friendly"
        }
    ],
    "league": {
        "iconUrls": {
            "medium": "https://api-assets.clashofclans.com/leagues/288/qVCZmeYH0lS7Gaa6YoB7LrNly7bfw7fV_d4Vp2CU-gk.png",
            "small": "https://api-assets.clashofclans.com/leagues/72/qVCZmeYH0lS7Gaa6YoB7LrNly7bfw7fV_d4Vp2CU-gk.png",
            "tiny": "https://api-assets.clashofclans.com/leagues/36/qVCZmeYH0lS7Gaa6YoB7LrNly7bfw7fV_d4Vp2CU-gk.png"
        },
        "id": 29000021,
        "name": "Titan League I"
    },
    "legendStatistics": {
        "bestSeason": {
            "id": "2022-05",
            "rank": 545758,
            "trophies": 5031
        },
        "currentSeason": {
            "trophies": 4865
        },
        "legendTrophies": 31,
        "previousSeason": {
            "id": "2022-05",
            "rank": 545758,
            "trophies": 5031
        }
    },
    "name": "ABCdef",
    "role": "leader",
    "spells": [
        {
            "level": 9,
            "maxLevel": 9,
            "name": "Lightning Spell",
            "village": "home"
        },
        {
            "level": 7,
            "maxLevel": 8,
            "name": "Healing Spell",
            "village": "home"
        },
        {
            "level": 6,
            "maxLevel": 6,
            "name": "Rage Spell",
            "village": "home"
        },
        {
            "level": 3,
            "maxLevel": 4,
            "name": "Jump Spell",
            "village": "home"
        },
        {
            "level": 7,
            "maxLevel": 7,
            "name": "Freeze Spell",
            "village": "home"
        },
        {
            "level": 4,
            "maxLevel": 8,
            "name": "Poison Spell",
            "village": "home"
        },
        {
            "level": 5,
            "maxLevel": 5,
            "name": "Earthquake Spell",
            "village": "home"
        },
        {
            "level": 5,
            "maxLevel": 5,
            "name": "Haste Spell",
            "village": "home"
        },
        {
            "level": 3,
            "maxLevel": 7,
            "name": "Clone Spell",
            "village": "home"
        },
        {
            "level": 2,
            "maxLevel": 7,
            "name": "Skeleton Spell",
            "village": "home"
        },
        {
            "level": 4,
            "maxLevel": 5,
            "name": "Bat Spell",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 4,
            "name": "Invisibility Spell",
            "village": "home"
        }
    ],
    "tag": "#LVGV0CJC",
    "townHallLevel": 13,
    "townHallWeaponLevel": 5,
    "troops": [
        {
            "level": 8,
            "maxLevel": 10,
            "name": "Barbarian",
            "village": "home"
        },
        {
            "level": 8,
            "maxLevel": 10,
            "name": "Archer",
            "village": "home"
        },
        {
            "level": 7,
            "maxLevel": 8,
            "name": "Goblin",
            "village": "home"
        },
        {
            "level": 10,
            "maxLevel": 10,
            "name": "Giant",
            "village": "home"
        },
        {
            "level": 8,
            "maxLevel": 10,
            "name": "Wall Breaker",
            "village": "home"
        },
        {
            "level": 9,
            "maxLevel": 10,
            "name": "Balloon",
            "village": "home"
        },
        {
            "level": 9,
            "maxLevel": 10,
            "name": "Wizard",
            "village": "home"
        },
        {
            "level": 4,
            "maxLevel": 7,
            "name": "Healer",
            "village": "home"
        },
        {
            "level": 8,
            "maxLevel": 9,
            "name": "Dragon",
            "village": "home"
        },
        {
            "level": 8,
            "maxLevel": 9,
            "name": "P.E.K.K.A",
            "village": "home"
        },
        {
            "level": 6,
            "maxLevel": 10,
            "name": "Minion",
            "village": "home"
        },
        {
            "level": 6,
            "maxLevel": 11,
            "name": "Hog Rider",
            "village": "home"
        },
        {
            "level": 7,
            "maxLevel": 9,
            "name": "Valkyrie",
            "village": "home"
        },
        {
            "level": 4,
            "maxLevel": 11,
            "name": "Golem",
            "village": "home"
        },
        {
            "level": 5,
            "maxLevel": 5,
            "name": "Witch",
            "village": "home"
        },
        {
            "level": 4,
            "maxLevel": 6,
            "name": "Lava Hound",
            "village": "home"
        },
        {
            "level": 4,
            "maxLevel": 6,
            "name": "Bowler",
            "village": "home"
        },
        {
            "level": 4,
            "maxLevel": 8,
            "name": "Baby Dragon",
            "village": "home"
        },
        {
            "level": 5,
            "maxLevel": 8,
            "name": "Miner",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 3,
            "name": "Super Barbarian",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 3,
            "name": "Super Archer",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 4,
            "name": "Super Wall Breaker",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 2,
            "name": "Super Giant",
            "village": "home"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Raged Barbarian",
            "village": "builderBase"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Sneaky Archer",
            "village": "builderBase"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Beta Minion",
            "village": "builderBase"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Boxer Giant",
            "village": "builderBase"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Bomber",
            "village": "builderBase"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Super P.E.K.K.A",
            "village": "builderBase"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Cannon Cart",
            "village": "builderBase"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Drop Ship",
            "village": "builderBase"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Baby Dragon",
            "village": "builderBase"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Night Witch",
            "village": "builderBase"
        },
        {
            "level": 3,
            "maxLevel": 4,
            "name": "Wall Wrecker",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 4,
            "name": "Battle Blimp",
            "village": "home"
        },
        {
            "level": 2,
            "maxLevel": 4,
            "name": "Yeti",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 2,
            "name": "Sneaky Goblin",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 3,
            "name": "Rocket Balloon",
            "village": "home"
        },
        {
            "level": 4,
            "maxLevel": 6,
            "name": "Ice Golem",
            "village": "home"
        },
        {
            "level": 4,
            "maxLevel": 5,
            "name": "Electro Dragon",
            "village": "home"
        },
        {
            "level": 2,
            "maxLevel": 4,
            "name": "Stone Slammer",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 3,
            "name": "Inferno Dragon",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 3,
            "name": "Super Valkyrie",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 1,
            "name": "Super Witch",
            "village": "home"
        },
        {
            "level": 18,
            "maxLevel": 18,
            "name": "Hog Glider",
            "village": "builderBase"
        },
        {
            "level": 1,
            "maxLevel": 2,
            "name": "Ice Hound",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 3,
            "name": "Super Bowler",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 3,
            "name": "Super Dragon",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 3,
            "name": "Headhunter",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 2,
            "name": "Super Wizard",
            "village": "home"
        },
        {
            "level": 1,
            "maxLevel": 3,
            "name": "Super Minion",
            "village": "home"
        }
    ],
    "trophies": 4865,
    "versusBattleWinCount": 3798,
    "versusBattleWins": 3798,
    "versusTrophies": 4071,
    "warPreference": "in",
    "warStars": 652
}
"##;

#[test]
fn player() {
    let player: Player = json::from_str(PLAYER_JSON).unwrap();
    assert_eq!(player.name, "ABCdef");
}
