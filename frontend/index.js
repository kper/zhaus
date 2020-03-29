import * as wasm from 'zhaus';
import L from 'leaflet';
import $ from 'jquery';

var reactor = wasm.get_reactor();

var map = L.map("map-container", {
}).setView([47.56667, 14.23333], 7);

map.zoomControl.setPosition('bottomright') // Position zoom control

var southWest = L.latLng(44, 9),
    northEast = L.latLng(50, 18);
var bounds = L.latLngBounds(southWest, northEast);

map.setMaxBounds(bounds);
map.on('drag', function () {
    map.panInsideBounds(bounds, { animate: false });
});



function onEachFeature(feature, layer) {
    if (feature.properties) {

        var infected = feature.properties.infected ? feature.properties.infected : 0;
        var dead = feature.properties.dead ? feature.properties.dead : 0;

        var infected_color = "#CC0033";

        var getColor = function (infected, dead) {
            console.log(infected);

            if (infected == 0) {
                return "#A9A9A9";
            }
            else {
                return infected_color;
            }
        }


        var getOpacity = function (infected, dead) {
            if (infected < 10) {
                return 0.1;
            }
            else if (infected < 100) {
                return 0.2;
            }
            else if (infected < 1000) {
                return 0.4;
            }
            else if (infected < 10000) {
                return 0.7;
            }
            else {
                return 1.0;
            }
        };

        var color = getColor(infected, dead);
        var op = getOpacity(infected, dead);

        var defaultStyle = {
            "color": "#A9A9A9",
            "weight": 1,
            "opacity": 0.5
        };

        if (infected == 0) {
            layer.setStyle(defaultStyle);
        }
        else {
            layer.setStyle({
                "color": color,
                "weight": 2,
                "opacity": op
            });
        }

        var popup_content = "<h4>" + feature.properties.name + "</h4><br/>" +
            "Infected: " + infected + "</br>" +
            "Dead:" + dead + "</br>" +
            "<button ref=\"" + feature.properties.name + "\" class='btn_limit'>Ausgangsbeschränkung</button><br/>" +
            "<button ref=\"" + feature.properties.name + "\" class='btn_lockdown'>Ausgangsperre</button><br/>" +
            "<button ref=\"" + feature.properties.name + "\" class='btn_qua'>Quarantäne</button><br/>";

        if (feature.properties.name == "Landeck") {
            popup_content += "<br/>" +
                "<button ref=\"" + feature.properties.name + "\" class='btn_ischgl'>Schigebiete eröffnen</button>";
        }

        layer.bindPopup(popup_content);
    }
    else {
        console.log('skip');
    }
};

$("body").on('click', '.btn_qua', function (e) {
    e.preventDefault();

    var name = $(this).attr("ref");

    reactor.action_quarantine(name);
});


var handle_infections = function (infected_districts) {
    map.eachLayer(function (layer) {
        map.removeLayer(layer);
    });

    L.tileLayer('https://{s}.tile.osm.org/{z}/{x}/{y}.png', {
        attribution: '&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors',
    }).addTo(map);

    var layer = L.geoJSON(null, {
        onEachFeature: onEachFeature
    }).addTo(map);

    for (var i of infected_districts) {
        let obj = Object.fromEntries(i);
        obj.geometry = Object.fromEntries(obj.geometry);
        obj.properties = Object.fromEntries(obj.properties);

        layer.addData(obj);
    }
};

var infected_districts = reactor.get_overlay_infected();

console.log(infected_districts);

//Start
handle_infections(infected_districts);
//Start end


document.getElementById("turn").onclick = function () {
    reactor.tick();

    var infected_districts = reactor.get_overlay_infected();

    handle_infections(infected_districts);
}