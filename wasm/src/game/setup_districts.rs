use super::{Game, District};

pub(crate) fn setup_districts() -> Vec<District> {
    let mut map = Vec::new();

    Game::create_district(
        &mut map,
        "Bludenz",
        vec!["Feldkirch", "Bregenz", "Reutte", "Landeck"],
    );
    Game::create_district(
        &mut map,
        "Wels-Land",
        vec![
            "Wels(Stadt)",
            "Grieskirchen",
            "Vöcklabruck",
            "Gmunden",
            "Kirchdorf an der Krems",
            "Steyr Land",
            "Linz-Land",
            "Eferding",
        ],
    );
    Game::create_district(
        &mut map,
        "Salzburg-Umgebung",
        vec![
            "Salzburg(Stadt)",
            "Braunau am Inn",
            "Vöcklabruck",
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
            "Völkermarkt",
        ],
    );
    Game::create_district(
        &mut map,
        "Korneuburg",
        vec!["Hollabrunn", "Mistelbach", "Wien(Stadt)", "Tulln"],
    );
    Game::create_district(
        &mut map,
        "Freistadt",
        vec!["Urfahr-Umgebung", "Gmünd", "Zwettl", "Perg"],
    );
    Game::create_district(
        &mut map,
        "Wiener Neustadt(Stadt)",
        vec!["Wiener Neustadt(Land)", "Mattersburg", "Neunkirchen"],
    );
    Game::create_district(
        &mut map,
        "Linz-Land",
        vec![
            "Linz(Stadt)",
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
            "Gmünd",
            "Freistadt",
            "Perg",
            "Melk",
            "Krems(Land)",
            "Horn",
        ],
    );
    Game::create_district(
        &mut map,
        "Neusiedl am See",
        vec!["Bruck an der Leitha", "Eisenstadt-Umgebung", "Rust(Stadt)"],
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
            "Krems(Land)",
            "Zwettl",
            "Perg",
            "Amstetten",
            "Scheibbs",
            "Sankt Pölten(Land)",
            "Krems(Land)",
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
            "Bruck-Mürzzuschlag",
            "Scheibbs",
            "Amstetten",
            "Steyr-Land",
        ],
    );
    Game::create_district(
        &mut map,
        "Völkermarkt",
        vec!["Klagenfurt Land", "Sankt Veit an der Glan", "Wolfsberg"],
    );
    Game::create_district(
        &mut map,
        "Horn",
        vec![
            "Waidhofen an der Thaya",
            "Zwettl",
            "Krems(Land)",
            "Hollabrunn",
        ],
    );
    Game::create_district(
        &mut map,
        "Waidhofen an der Thaya",
        vec!["Horn", "Zwettl", "Gmünd"],
    );
    Game::create_district(&mut map, "Kufstein", vec!["Schwaz", "Kitzbühel"]);
    Game::create_district(
        &mut map,
        "Eferding",
        vec![
            "Rohrbach",
            "Urfahr-Umgebung",
            "Linz-Land",
            "Wels-Land",
            "Grieskirchen",
            "Schärding",
        ],
    );
    Game::create_district(
        &mut map,
        "Amstetten",
        vec![
            "Perg",
            "Melk",
            "Scheibbs",
            "Waidhofen an der Ybbs",
            "Steyr-Land",
            "Steyr(Stadt)",
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
            "Linz(Stadt)",
            "Linz-Land",
            "Melk",
            "Zwettl",
        ],
    );
    Game::create_district(
        &mut map,
        "Sankt Pölten(Land)",
        vec![
            "Sankt Pölten(Stadt)",
            "Tulln",
            "Wien(Stadt)",
            "Mödling",
            "Baden",
            "Lilienfeld",
            "Melk",
            "Krems-Land",
            "Krems an der Donau(Stadt)",
        ],
    );
    Game::create_district(&mut map, "Innsbruck-Stadt", vec!["Innsbruck-Land"]);
    Game::create_district(
        &mut map,
        "Güssing",
        vec!["Oberwart", "Hartberg-Fürstenfeld", "Südoststeiermark"],
    );
    Game::create_district(
        &mut map,
        "Deutschlandsberg",
        vec!["Voitsberg", "Graz-Umgebung", "Leibnitz", "Wolfsberg"],
    );
    Game::create_district(&mut map, "Salzburg(Stadt)", vec!["Salzburg-Umgebung"]);
    Game::create_district(
        &mut map,
        "Leibnitz",
        vec!["Graz-Umgebung", "Deutschlandsberg", "Südoststeiermark"],
    );
    Game::create_district(
        &mut map,
        "Lienz",
        vec!["Spittal an der Drau", "Hermagor", "Zell am See"],
    );
    Game::create_district(
        &mut map,
        "Hollabrunn",
        vec!["Horn", "Krems(Land)", "Tulln", "Korneuburg", "Mistelbach"],
    );
    Game::create_district(
        &mut map,
        "Wiener Neustadt(Land)",
        vec![
            "Wiener Neustadt(Stadt)",
            "Neunkirchen",
            "Baden",
            "Eisenstadt-Umgebung",
            "Mattersburg",
            "Oberpullendorf",
            "Oberwart",
            "Hartberg-Fürstenfeld",
        ],
    );
    Game::create_district(
        &mut map,
        "Braunau am Inn",
        vec!["Ried im Innkreis", "Vöcklbruck", "Salzburg-Umgebung"],
    );
    Game::create_district(
        &mut map,
        "Baden",
        vec![
            "Mödling",
            "Bruck an der Leitha",
            "Eisenstadt-Umgebung",
            "Wiener Neustadt(Land)",
            "Lilienfeld",
            "Sankt Pölten(Land)",
        ],
    );
    Game::create_district(
        &mut map,
        "Ried im Innkreis",
        vec!["Braunau am Inn", "Schärding", "Grieskirchen", "Vöcklabruck"],
    );
    Game::create_district(
        &mut map,
        "Hermagor",
        vec!["Spittal an der Drau", "Villach Land", "Lienz"],
    );
    Game::create_district(
        &mut map,
        "Neunkirchen",
        vec![
            "Wiener Neustadt(Land)",
            "Wiener Neustadt(Stadt)",
            "Hartberg-Fürstenfeld",
            "Weiz",
            "Bruck-Mürzzuschlag",
            "Lilienfeld",
        ],
    );
    Game::create_district(
        &mut map,
        "Jennersdorf",
        vec!["Hartberg-Fürstenfeld", "Güssing", "Südoststeiermark"],
    );
    Game::create_district(
        &mut map,
        "Zell am See",
        vec![
            "Lienz",
            "Spittal an der Drau",
            "Sankt Johann im Pongau",
            "Kitzbühel",
            "Schwaz",
        ],
    );
    Game::create_district(
        &mut map,
        "Krems(Land)",
        vec![
            "Horn",
            "Zwettl",
            "Melk",
            "Sankt Pölten(Land)",
            "Krems an der Donau(Stadt)",
            "Tulln",
            "Hollabrunn",
        ],
    );
    Game::create_district(
        &mut map,
        "Leoben",
        vec!["Murtal", "Liezen", "Bruck-Mürzzuschlag", "Graz-Umgebung"],
    );
    Game::create_district(
        &mut map,
        "Mödling",
        vec![
            "Sankt Pölten(Land)",
            "Wien(Stadt)",
            "Bruck an der Leitha",
            "Baden",
        ],
    );
    Game::create_district(
        &mut map,
        "Scheibbs",
        vec![
            "Amstetten",
            "Waidhofen an der Ybbs",
            "Melk",
            "Sankt Pölten(Land)",
            "Lilienfeld",
            "Bruck-Mürzzuschlag",
            "Liezen",
        ],
    );
    Game::create_district(
        &mut map,
        "Rust(Stadt)",
        vec!["Eisenstadt-Umgebung", "Neusiedl am See"],
    );
    Game::create_district(&mut map, "Wels(Stadt)", vec!["Wels Land"]);
    Game::create_district(
        &mut map,
        "Bruck an der Leitha",
        vec![
            "Gänserndorf",
            "Wien(Stadt)",
            "Mödling",
            "Baden",
            "Eisenstadt-Umgebung",
            "Neusiedl am See",
        ],
    );
    Game::create_district(
        &mut map,
        "Rohrbach",
        vec!["Eferding", "Grieskirchen", "Schärding", "Urfahr-Umgebung"],
    );
    Game::create_district(
        &mut map,
        "Villach Stadt",
        vec!["Villach Land", "Feldkirchen"],
    );
    Game::create_district(
        &mut map,
        "Reutte",
        vec!["Imst", "Landeck", "Bludenz", "Bregenz"],
    );
    Game::create_district(
        &mut map,
        "Schärding",
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
        vec!["Gänserndorf", "Korneuburg", "Hollabrunn"],
    );
    Game::create_district(
        &mut map,
        "Tulln",
        vec![
            "Hollabrunn",
            "Korneuburg",
            "Wien(Stadt)",
            "Sankt Pölten(Land)",
            "Krems an der Donau(Stadt)",
            "Krems(Land)",
        ],
    );
    Game::create_district(
        &mut map,
        "Eisenstadt-Umgebung",
        vec![
            "Eisenstadt(Stadt)",
            "Bruck an der Leitha",
            "Neusiedl am See",
            "Rust(Stadt)",
            "Mattersburg",
            "Wiener Neustadt(Land)",
            "Baden",
        ],
    );
    Game::create_district(&mut map, "Steyr(Stadt)", vec!["Amstetten", "Steyr-Land"]);
    Game::create_district(
        &mut map,
        "Sankt Veit an der Glan",
        vec![
            "Feldkirchen",
            "Murau",
            "Wolfsberg",
            "Völkermarkt",
            "Klagenfurt Land",
            "Klagenfurt Stadt",
            "Murtal",
        ],
    );
    Game::create_district(
        &mut map,
        "Gänserndorf",
        vec![
            "Mistelbach",
            "Korneuburg",
            "Wien(Stadt)",
            "Bruck an der Leitha",
        ],
    );
    Game::create_district(
        &mut map,
        "Schwaz",
        vec!["Innsbruck-Land", "Kufstein", "Kitzbühel", "Zell am See"],
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
        "Wien(Stadt)",
        vec![
            "Korneuburg",
            "Gänserndorf",
            "Bruck an der Leitha",
            "Mödling",
            "Sankt Pölten(Land)",
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
        "Krems an der Donau(Stadt)",
        vec!["Krems(Land)", "Sankt Pölten(Land)"],
    );
    Game::create_district(
        &mut map,
        "Feldkirchen",
        vec![
            "Villach Land",
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
            "Steyr(Stadt)",
            "Amstetten",
            "Waidhofen an der Ybbs(Stadt)",
            "Liezen",
            "Kirchdorf an der Krems",
            "Wels-Land",
        ],
    );
    Game::create_district(
        &mut map,
        "Weiz",
        vec![
            "Bruck-Mürzzuschlag",
            "Neunkirchen",
            "Hartberg-Fürstenfeld",
            "Südoststeiermark",
            "Graz-Umgebung",
        ],
    );
    Game::create_district(
        &mut map,
        "Imst",
        vec!["Landeck", "Reutte", "Innsbruck-Land"],
    );
    Game::create_district(
        &mut map,
        "Südoststeiermark",
        vec![
            "Leibnitz",
            "Graz-Umgebung",
            "Weiz",
            "Hartberg-Fürstenfeld",
            "Güssing",
        ],
    );
    Game::create_district(
        &mut map,
        "Linz(Stadt)",
        vec!["Urfahr-Umgebung", "Perg", "Linz-Land"],
    );
    Game::create_district(
        &mut map,
        "Gmünd",
        vec!["Waidhofen an der Thaya", "Zwettl", "Freistadt"],
    );
    Game::create_district(
        &mut map,
        "Mattersburg",
        vec![
            "Eisenstadt-Umgebung",
            "Wiener Neustadt(Stadt)",
            "Wiener Neustadt(Land)",
            "Oberpullendorf",
        ],
    );
    Game::create_district(
        &mut map,
        "Klagenfurt Land",
        vec![
            "Villach Land",
            "Feldkirchen",
            "Sankt Veit an der Glan",
            "Völkermarkt",
        ],
    );
    Game::create_district(
        &mut map,
        "Oberpullendorf",
        vec!["Mattersburg", "Wiener Neustadt(Land)", "Oberwart"],
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
            "Schärding",
            "Ried im Innkreis",
            "Vöcklabruck",
            "Wels-Land",
        ],
    );
    Game::create_district(&mut map, "Sankt Pölten(Stadt)", vec!["Sankt Pölten(Land)"]);
    Game::create_district(
        &mut map,
        "Urfahr-Umgebung",
        vec![
            "Rohrbach",
            "Freistadt",
            "Linz(Stadt)",
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
    Game::create_district(&mut map, "Eisenstadt(Stadt)", vec!["Eisenstadt-Umgebung"]);
    Game::create_district(
        &mut map,
        "Hartberg-Fürstenfeld",
        vec![
            "Oberwart",
            "Güssing",
            "Südoststeiermark",
            "Weiz",
            "Neunkirchen",
            "Wiener Neustadt(Land)",
        ],
    );
    Game::create_district(
        &mut map,
        "Graz-Umgebung",
        vec![
            "Bruck-Mürzzuschlag",
            "Weiz",
            "Graz(Stadt)",
            "Voitsberg",
            "Deutschlandsberg",
            "Leibnitz",
            "Südoststeiermark",
            "Leoben",
        ],
    );
    Game::create_district(
        &mut map,
        "Villach Land",
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
            "Güssing",
            "Hartberg-Fürstenfeld",
            "Wiener Neustadt(Land)",
            "Oberpullendorf",
        ],
    );
    Game::create_district(
        &mut map,
        "Bregenz",
        vec!["Dornbirn", "Bludenz", "Reutte", "Feldkirch"],
    );
    Game::create_district(
        &mut map,
        "Lilienfeld",
        vec![
            "Sankt Pölten(Land)",
            "Baden",
            "Wiener Neustadt(Land)",
            "Neunkirchen",
            "Bruck-Mürzzuschlag",
            "Scheibbs",
        ],
    );
    Game::create_district(
        &mut map,
        "Hallein",
        vec!["Salzburg-Umgebung", "Gmunden", "Sankt Johann im Pongau"],
    );
    Game::create_district(&mut map, "Kitzbühel", vec!["Kufstein", "Zell am See"]);
    Game::create_district(
        &mut map,
        "Gmunden",
        vec![
            "Vöcklabruck",
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
        "Waidhofen an der Ybbs(Stadt)",
        vec!["Amstetten", "Scheibbs", "Steyr-Land"],
    );
    Game::create_district(
        &mut map,
        "Vöcklabruck",
        vec![
            "Ried im Innkreis",
            "Braunau am Inn",
            "Salzburg-Umgebung",
            "Gmunden",
            "Wels-Land",
            "Grieskirchen",
        ],
    );
    Game::create_district(&mut map, "Landeck", vec!["Imst", "Reutte", "Bludenz"]);
    Game::create_district(
        &mut map,
        "Bruck-Mürzzuschlag",
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
    Game::create_district(&mut map, "Graz(Stadt)", vec!["Graz-Umgebung"]);
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
            "Villach Land",
            "Feldkirchen",
            "Murau",
            "Tamsweg",
        ],
    );

    map
}
