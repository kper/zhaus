import * as wasm from 'zhaus';
import L from 'leaflet';

var map = L.map("map-container", {
}).setView([47.56667, 14.23333], 7);

map.zoomControl.setPosition('bottomright') // Position zoom control
var layers = {}; // Map layer dict (key/value = title/layer)
var selectedRegion = null; // Store currently selected region

L.tileLayer('https://{s}.tile.osm.org/{z}/{x}/{y}.png', {
    attribution: '&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors',
}).addTo(map);

var reactor = wasm.get_reactor();

var layer_districts;

var infected_color = "#CC0033";

var getOpacity = function(infected, dead) {
    console.log('infected', infected);
    if(infected < 10) {
        return 0.1;
    }
    else if(infected < 30) {
        return 0.2;
    }
    else if(infected < 100) {
        return 0.4;
    }
    else if(infected < 500) {
        return 0.7;
    }
    else {
        return 0.9;
    }
};

var xhr = new XMLHttpRequest();
xhr.open('GET', 'bezirke.json');
xhr.setRequestHeader('Content-Type', 'application/json');
xhr.onload = function () {
    if (xhr.status === 200) {
        var defaultStyle = {
            "color": "#A9A9A9",
            "weight": 2,
            "opacity": 0.65
        };

        L.geoJSON(JSON.parse(xhr.responseText), {
            style: defaultStyle
        }).addTo(map);

        layer_districts = L.geoJSON().addTo(map);
        //layer_districts.style = infectedStyle;

        var infected_districts = reactor.get_overlay_infected();

        //Start
        for (var i of infected_districts) {
            let obj = Object.fromEntries(i);
            obj.geometry = Object.fromEntries(obj.geometry);
            obj.properties = Object.fromEntries(obj.properties);

            var infectedStyle = {
                "color": infected_color,
                "weight": 1,
                "opacity": getOpacity(obj.properties.infected, obj.properties.dead)
            };

            layer_districts.addData(obj);
            layer_districts.setStyle(infectedStyle);
        }
        //Start end
    }
};
xhr.send();

document.getElementById("turn").onclick = function () {
    map.removeLayer(layer_districts);
    layer_districts = L.geoJSON().addTo(map);

    console.log('tick');
    reactor.tick();
    console.log(reactor.to_string());

    var infected_districts = reactor.get_overlay_infected();

    for (var i of infected_districts) {
        let obj = Object.fromEntries(i);
        obj.geometry = Object.fromEntries(obj.geometry);
        obj.properties = Object.fromEntries(obj.properties);

        var infectedStyle = {
                "color": infected_color,
                "weight": 1,
                "opacity": getOpacity(obj.properties.infected, obj.properties.dead)
            };

        layer_districts.addData(obj);
        layer_districts.setStyle(infectedStyle);
    }
}