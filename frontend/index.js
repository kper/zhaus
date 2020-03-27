import * as wasm from 'zhaus';
import L from 'leaflet';

console.log(wasm.greet("Kevin"));

var map = L.map("map-container", {
}).setView([47.56667, 14.23333], 7);

map.zoomControl.setPosition('bottomright') // Position zoom control
var layers = {}; // Map layer dict (key/value = title/layer)
var selectedRegion = null; // Store currently selected region

L.tileLayer('https://{s}.tile.osm.org/{z}/{x}/{y}.png', {
    attribution: '&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors',
}).addTo(map);

var xhr = new XMLHttpRequest();
xhr.open('GET', 'bezirke.json');
xhr.setRequestHeader('Content-Type', 'application/json');
xhr.onload = function () {
    if (xhr.status === 200) {
        L.geoJSON(JSON.parse(xhr.responseText)).addTo(map);
    }
};
xhr.send();