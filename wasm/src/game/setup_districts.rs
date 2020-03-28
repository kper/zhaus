use super::{Game, District};

pub(crate) fn setup_districts() -> Vec<District> {
    let mut map = Vec::new();

    Game::create_district(
        &mut map,
        "Bludenz",
        vec!["Feldkirch", "Bregenz", "Reutte (Ausserfern)", "Landeck"],
    );
    Game::create_district(
        &mut map,
        "Wels-Land",
        vec![
            "Wels (Stadt)",
            "Grieskirchen",
            "Voecklabruck",
            "Gmunden",
            "Kirchdorf an der Krems",
            "Steyr-Land",
            "Linz-Land",
            "Eferding",
        ],
    );
    Game::create_district(
        &mut map,
        "Salzburg-Umgebung",
        vec![
            "Salzburg (Stadt)",
            "Braunau am Inn",
            "Voecklabruck",
            "Gmunden",
            "Hallein",
        ],
    );
    Game::create_district(
        &mut map,
        "Wolfsberg",
        vec![
            "Sankt Veit an der Glan",
            "Murtal",
            "Voitsberg",
            "Deutschlandsberg",
            "Voelkermarkt",
        ],
    );
    Game::create_district(
        &mut map,
        "Korneuburg",
        vec!["Hollabrunn", "Mistelbach", "Wien", "Tulln"],
    );
    Game::create_district(
        &mut map,
        "Freistadt",
        vec!["Urfahr-Umgebung", "Gmuend", "Zwettl", "Perg"],
    );
    Game::create_district(
        &mut map,
        "Wiener Neustadt (Stadt)",
        vec!["Wiener Neustadt (Land)", "Mattersburg", "Neunkirchen"],
    );
    Game::create_district(
        &mut map,
        "Linz-Land",
        vec![
            "Linz (Stadt)",
            "Urfahr-Umgebung",
            "Wels-Land",
            "Steyr-Land",
            "Amstetten",
            "Perg",
            "Eferding",
        ],
    );
    Game::create_district(
        &mut map,
        "Klagenfurt Stadt",
        vec!["Klagenfurt Land", "Sankt Veit an der Glan", "Feldkirchen"],
    );
    Game::create_district(
        &mut map,
        "Zwettl",
        vec![
            "Waidhofen an der Thaya",
            "Gmuend",
            "Freistadt",
            "Perg",
            "Melk",
            "Krems (Land)",
            "Horn",
        ],
    );
    Game::create_district(
        &mut map,
        "Neusiedl am See",
        vec!["Bruck an der Leitha", "Eisenstadt-Umgebung", "Rust (Stadt)"],
    );
    Game::create_district(
        &mut map,
        "Murtal",
        vec![
            "Liezen",
            "Leoben",
            "Graz-Umgebung",
            "Voitsberg",
            "Wolfsberg",
            "Sankt Veit an der Glan",
            "Murau",
        ],
    );
    Game::create_district(
        &mut map,
        "Melk",
        vec![
            "Krems (Land)",
            "Zwettl",
            "Perg",
            "Amstetten",
            "Scheibbs",
            "Sankt Poelten (Land)",
        ],
    );
    Game::create_district(
        &mut map,
        "Liezen",
        vec![
            "Kirchdorf an der Krems",
            "Gmunden",
            "Sankt Johann im Pongau",
            "Tamsweg",
            "Murau",
            "Murtal",
            "Leoben",
            "Bruck-Muerzzuschlag",
            "Scheibbs",
            "Amstetten",
            "Steyr-Land",
        ],
    );
    Game::create_district(
        &mut map,
        "Voelkermarkt",
        vec!["Klagenfurt Land", "Sankt Veit an der Glan", "Wolfsberg"],
    );
    Game::create_district(
        &mut map,
        "Horn",
        vec![
            "Waidhofen an der Thaya",
            "Zwettl",
            "Krems (Land)",
            "Hollabrunn",
        ],
    );
    Game::create_district(
        &mut map,
        "Waidhofen an der Thaya",
        vec!["Horn", "Zwettl", "Gmuend"],
    );
    Game::create_district(&mut map, "Kufstein", vec!["Schwaz", "Kitzbuehel"]);
    Game::create_district(
        &mut map,
        "Eferding",
        vec![
            "Rohrbach",
            "Urfahr-Umgebung",
            "Linz-Land",
            "Wels-Land",
            "Grieskirchen",
            "Schaerding",
        ],
    );
    Game::create_district(
        &mut map,
        "Amstetten",
        vec![
            "Perg",
            "Melk",
            "Scheibbs",
            "Waidhofen an der Ybbs (Stadt)",
            "Steyr-Land",
            "Steyr (Stadt)",
            "Linz-Land",
            "Liezen",
        ],
    );
    Game::create_district(
        &mut map,
        "Perg",
        vec![
            "Amstetten",
            "Freistadt",
            "Urfahr-Umgebung",
            "Linz (Stadt)",
            "Linz-Land",
            "Melk",
            "Zwettl",
        ],
    );
    Game::create_district(
        &mut map,
        "Sankt Poelten (Land)",
        vec![
            //"Sankt Poelten(Stadt)",
            "Tulln",
            "Wien",
            "Moedling",
            "Baden",
            "Lilienfeld",
            "Melk",
            "Krems (Land)",
            "Krems an der Donau (Stadt)",
        ],
    );
    Game::create_district(&mut map, "Innsbruck-Stadt", vec!["Innsbruck-Land"]);
    Game::create_district(
        &mut map,
        "Guessing",
        vec!["Oberwart", "Hartberg-Fuerstenfeld", "Suedoststeiermark"],
    );
    Game::create_district(
        &mut map,
        "Deutschlandsberg",
        vec!["Voitsberg", "Graz-Umgebung", "Leibnitz", "Wolfsberg"],
    );
    Game::create_district(&mut map, "Salzburg (Stadt)", vec!["Salzburg-Umgebung"]);
    Game::create_district(
        &mut map,
        "Leibnitz",
        vec!["Graz-Umgebung", "Deutschlandsberg", "Suedoststeiermark"],
    );
    Game::create_district(
        &mut map,
        "Lienz",
        vec!["Spittal an der Drau", "Hermagor", "Zell am See"],
    );
    Game::create_district(
        &mut map,
        "Hollabrunn",
        vec!["Horn", "Krems (Land)", "Tulln", "Korneuburg", "Mistelbach"],
    );
    Game::create_district(
        &mut map,
        "Wiener Neustadt (Land)",
        vec![
            "Wiener Neustadt (Stadt)",
            "Neunkirchen",
            "Baden",
            "Eisenstadt-Umgebung",
            "Mattersburg",
            "Oberpullendorf",
            "Oberwart",
            "Hartberg-Fuerstenfeld",
        ],
    );
    Game::create_district(
        &mut map,
        "Braunau am Inn",
        vec!["Ried im Innkreis", "Voecklabruck", "Salzburg-Umgebung"],
    );
    Game::create_district(
        &mut map,
        "Baden",
        vec![
            "Moedling",
            "Bruck an der Leitha",
            "Eisenstadt-Umgebung",
            "Wiener Neustadt (Land)",
            "Lilienfeld",
            "Sankt Poelten (Land)",
        ],
    );
    Game::create_district(
        &mut map,
        "Ried im Innkreis",
        vec!["Braunau am Inn", "Schaerding", "Grieskirchen", "Voecklabruck"],
    );
    Game::create_district(
        &mut map,
        "Hermagor",
        vec!["Spittal an der Drau", "Villach-Land", "Lienz"],
    );
    Game::create_district(
        &mut map,
        "Neunkirchen",
        vec![
            "Wiener Neustadt (Land)",
            "Wiener Neustadt (Stadt)",
            "Hartberg-Fuerstenfeld",
            "Weiz",
            "Bruck-Muerzzuschlag",
            "Lilienfeld",
        ],
    );
    Game::create_district(
        &mut map,
        "Jennersdorf",
        vec!["Hartberg-Fuerstenfeld", "Guessing", "Suedoststeiermark"],
    );
    Game::create_district(
        &mut map,
        "Zell am See",
        vec![
            "Lienz",
            "Spittal an der Drau",
            "Sankt Johann im Pongau",
            "Kitzbuehel",
            "Schwaz",
        ],
    );
    Game::create_district(
        &mut map,
        "Krems (Land)",
        vec![
            "Horn",
            "Zwettl",
            "Melk",
            "Sankt Poelten (Land)",
            "Krems an der Donau (Stadt)",
            "Tulln",
            "Hollabrunn",
        ],
    );
    Game::create_district(
        &mut map,
        "Leoben",
        vec!["Murtal", "Liezen", "Bruck-Muerzzuschlag", "Graz-Umgebung"],
    );
    Game::create_district(
        &mut map,
        "Moedling",
        vec![
            "Sankt Poelten (Land)",
            "Wien",
            "Bruck an der Leitha",
            "Baden",
        ],
    );
    Game::create_district(
        &mut map,
        "Scheibbs",
        vec![
            "Amstetten",
            "Waidhofen an der Ybbs (Stadt)",
            "Melk",
            "Sankt Poelten (Land)",
            "Lilienfeld",
            "Bruck-Muerzzuschlag",
            "Liezen",
        ],
    );
    Game::create_district(
        &mut map,
        "Rust (Stadt)",
        vec!["Eisenstadt-Umgebung", "Neusiedl am See"],
    );
    Game::create_district(&mut map, "Wels (Stadt)", vec!["Wels Land"]);
    Game::create_district(
        &mut map,
        "Bruck an der Leitha",
        vec![
            "Gaenserndorf",
            "Wien",
            "Moedling",
            "Baden",
            "Eisenstadt-Umgebung",
            "Neusiedl am See",
        ],
    );
    Game::create_district(
        &mut map,
        "Rohrbach",
        vec!["Eferding", "Grieskirchen", "Schaerding", "Urfahr-Umgebung"],
    );
    Game::create_district(
        &mut map,
        "Villach Stadt",
        vec!["Villach-Land", "Feldkirchen"],
    );
    Game::create_district(
        &mut map,
        "Reutte (Ausserfern)",
        vec!["Imst", "Landeck", "Bludenz", "Bregenz"],
    );
    Game::create_district(
        &mut map,
        "Schaerding",
        vec!["Ried im Innkreis", "Grieskirchen", "Rohrbach"],
    );
    Game::create_district(
        &mut map,
        "Voitsberg",
        vec!["Murtal", "Graz-Umgebung", "Deutschlandsberg", "Wolfsberg"],
    );
    Game::create_district(
        &mut map,
        "Mistelbach",
        vec!["Gaenserndorf", "Korneuburg", "Hollabrunn"],
    );
    Game::create_district(
        &mut map,
        "Tulln",
        vec![
            "Hollabrunn",
            "Korneuburg",
            "Wien",
            "Sankt Poelten (Land)",
            "Krems an der Donau (Stadt)",
            "Krems (Land)",
        ],
    );
    Game::create_district(
        &mut map,
        "Eisenstadt-Umgebung",
        vec![
            "Eisenstadt (Stadt)",
            "Bruck an der Leitha",
            "Neusiedl am See",
            "Rust (Stadt)",
            "Mattersburg",
            "Wiener Neustadt (Land)",
            "Baden",
        ],
    );
    Game::create_district(&mut map, "Steyr (Stadt)", vec!["Amstetten", "Steyr-Land"]);
    Game::create_district(
        &mut map,
        "Sankt Veit an der Glan",
        vec![
            "Feldkirchen",
            "Murau",
            "Wolfsberg",
            "Voelkermarkt",
            "Klagenfurt Land",
            "Klagenfurt Stadt",
            "Murtal",
        ],
    );
    Game::create_district(
        &mut map,
        "Gaenserndorf",
        vec![
            "Mistelbach",
            "Korneuburg",
            "Wien",
            "Bruck an der Leitha",
        ],
    );
    Game::create_district(
        &mut map,
        "Schwaz",
        vec!["Innsbruck-Land", "Kufstein", "Kitzbuehel", "Zell am See"],
    );
    Game::create_district(
        &mut map,
        "Sankt Johann im Pongau",
        vec![
            "Zell am See",
            "Hallein",
            "Gmunden",
            "Liezen",
            "Tamsweg",
            "Spittal an der Drau",
        ],
    );
    Game::create_district(
        &mut map,
        "Wien",
        vec![
            "Korneuburg",
            "Gaenserndorf",
            "Bruck an der Leitha",
            "Moedling",
            "Sankt Poelten (Land)",
            "Tulln",
            "Mistelbach",
        ],
    );
    Game::create_district(
        &mut map,
        "Kirchdorf an der Krems",
        vec!["Steyr-Land", "Wels-Land", "Linz-Land", "Gmunden"],
    );
    Game::create_district(
        &mut map,
        "Krems an der Donau (Stadt)",
        vec!["Krems (Land)", "Sankt Poelten (Land)"],
    );

    Game::create_district(
        &mut map,
        "Feldkirchen",
        vec![
            "Villach-Land",
            "Spittal an der Drau",
            "Murau",
            "Sankt Veit an der Glan",
            "Klagenfurt Land",
            "Klagenfurt Stadt",
            "Villach Stadt",
        ],
    );
    Game::create_district(
        &mut map,
        "Steyr-Land",
        vec![
            "Linz-Land",
            "Steyr (Stadt)",
            "Amstetten",
            "Waidhofen an der Ybbs (Stadt)",
            "Liezen",
            "Kirchdorf an der Krems",
            "Wels-Land",
        ],
    );
    Game::create_district(
        &mut map,
        "Weiz",
        vec![
            "Bruck-Muerzzuschlag",
            "Neunkirchen",
            "Hartberg-Fuerstenfeld",
            "Suedoststeiermark",
            "Graz-Umgebung",
        ],
    );
    Game::create_district(
        &mut map,
        "Imst",
        vec!["Landeck", "Reutte (Ausserfern)", "Innsbruck-Land"],
    );
    Game::create_district(
        &mut map,
        "Suedoststeiermark",
        vec![
            "Leibnitz",
            "Graz-Umgebung",
            "Weiz",
            "Hartberg-Fuerstenfeld",
            "Guessing",
        ],
    );
    Game::create_district(
        &mut map,
        "Linz (Stadt)",
        vec!["Urfahr-Umgebung", "Perg", "Linz-Land"],
    );
    Game::create_district(
        &mut map,
        "Gmuend",
        vec!["Waidhofen an der Thaya", "Zwettl", "Freistadt"],
    );
    Game::create_district(
        &mut map,
        "Mattersburg",
        vec![
            "Eisenstadt-Umgebung",
            "Wiener Neustadt (Stadt)",
            "Wiener Neustadt (Land)",
            "Oberpullendorf",
        ],
    );
    Game::create_district(
        &mut map,
        "Klagenfurt Land",
        vec![
            "Villach-Land",
            "Feldkirchen",
            "Sankt Veit an der Glan",
            "Voelkermarkt",
        ],
    );
    Game::create_district(
        &mut map,
        "Oberpullendorf",
        vec!["Mattersburg", "Wiener Neustadt (Land)", "Oberwart"],
    );
    Game::create_district(&mut map, "Dornbirn", vec!["Bregenz", "Feldkirch"]);
    Game::create_district(
        &mut map,
        "Tamsweg",
        vec![
            "Liezen",
            "Sankt Johann im Pongau",
            "Spittal an der Drau",
            "Murau",
        ],
    );
    Game::create_district(
        &mut map,
        "Grieskirchen",
        vec![
            "Eferding",
            "Schaerding",
            "Ried im Innkreis",
            "Voecklabruck",
            "Wels-Land",
        ],
    );
    //Game::create_district(&mut map, "Sankt Poelten (Stadt)", vec!["Sankt Poelten (Land)"]);
    Game::create_district(
        &mut map,
        "Urfahr-Umgebung",
        vec![
            "Rohrbach",
            "Freistadt",
            "Linz (Stadt)",
            "Linz-Land",
            "Eferding",
        ],
    );
    Game::create_district(
        &mut map,
        "Murau",
        vec![
            "Tamsweg",
            "Liezen",
            "Murtal",
            "Sankt Veit an der Glan",
            "Feldkirchen",
            "Spittal an der Drau",
        ],
    );
    Game::create_district(
        &mut map,
        "Innsbruck-Land",
        vec!["Innsbruck-Stadt", "Schwaz", "Imst"],
    );
    Game::create_district(&mut map, "Eisenstadt (Stadt)", vec!["Eisenstadt-Umgebung"]);
    Game::create_district(
        &mut map,
        "Hartberg-Fuerstenfeld",
        vec![
            "Oberwart",
            "Guessing",
            "Suedoststeiermark",
            "Weiz",
            "Neunkirchen",
            "Wiener Neustadt (Land)",
        ],
    );
    Game::create_district(
        &mut map,
        "Graz-Umgebung",
        vec![
            "Bruck-Muerzzuschlag",
            "Weiz",
            "Graz (Stadt)",
            "Voitsberg",
            "Deutschlandsberg",
            "Leibnitz",
            "Suedoststeiermark",
            "Leoben",
        ],
    );
    Game::create_district(
        &mut map,
        "Villach-Land",
        vec![
            "Spittal an der Drau",
            "Villach Stadt",
            "Feldkirch",
            "Klagenfurt Land",
            "Hermagor",
        ],
    );
    Game::create_district(
        &mut map,
        "Oberwart",
        vec![
            "Guessing",
            "Hartberg-Fuerstenfeld",
            "Wiener Neustadt (Land)",
            "Oberpullendorf",
        ],
    );
    Game::create_district(
        &mut map,
        "Bregenz",
        vec!["Dornbirn", "Bludenz", "Reutte (Ausserfern)", "Feldkirch"],
    );
    Game::create_district(
        &mut map,
        "Lilienfeld",
        vec![
            "Sankt Poelten (Land)",
            "Baden",
            "Wiener Neustadt (Land)",
            "Neunkirchen",
            "Bruck-Muerzzuschlag",
            "Scheibbs",
        ],
    );
    Game::create_district(
        &mut map,
        "Hallein",
        vec!["Salzburg-Umgebung", "Gmunden", "Sankt Johann im Pongau"],
    );
    Game::create_district(&mut map, "Kitzbuehel", vec!["Kufstein", "Zell am See"]);
    Game::create_district(
        &mut map,
        "Gmunden",
        vec![
            "Voecklabruck",
            "Kirchdorf an der Krems",
            "Liezen",
            "Sankt Johann im Pongau",
            "Hallein",
            "Salzburg-Umgebung",
            "Wels-Land",
        ],
    );
    Game::create_district(
        &mut map,
        "Waidhofen an der Ybbs (Stadt)",
        vec!["Amstetten", "Scheibbs", "Steyr-Land"],
    );
    Game::create_district(
        &mut map,
        "Voecklabruck",
        vec![
            "Ried im Innkreis",
            "Braunau am Inn",
            "Salzburg-Umgebung",
            "Gmunden",
            "Wels-Land",
            "Grieskirchen",
        ],
    );
    Game::create_district(&mut map, "Landeck", vec!["Imst", "Reutte (Ausserfern)", "Bludenz"]);
    Game::create_district(
        &mut map,
        "Bruck-Muerzzuschlag",
        vec![
            "Scheibbs",
            "Lilienfeld",
            "Neunkirchen",
            "Weiz",
            "Graz-Umgebung",
            "Leoben",
            "Liezen",
        ],
    );
    Game::create_district(&mut map, "Graz (Stadt)", vec!["Graz-Umgebung"]);
    Game::create_district(
        &mut map,
        "Feldkirch",
        vec!["Dornbirn", "Bregenz", "Bludenz"],
    );
    Game::create_district(
        &mut map,
        "Spittal an der Drau",
        vec![
            "Sankt Johann im Pongau",
            "Zell am See",
            "Lienz",
            "Hermagor",
            "Villach-Land",
            "Feldkirchen",
            "Murau",
            "Tamsweg",
        ],
    );

    map
}
