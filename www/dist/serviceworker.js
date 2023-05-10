importScripts('https://storage.googleapis.com/workbox-cdn/releases/5.0.0/workbox-sw.js');

const urls_to_cache = [
    "/seba2/www/dist/manifest.json",
    "/seba2/www/dist/0.bootstrap.js",
    "/seba2/www/dist/bootstrap.js",
    "/seba2/www/dist/index.html",
    new RegExp("/seba2/www/dist/.*\.wasm"),
]

for (let i = 0; i < urls_to_cache.length; i++) {
    workbox.routing.registerRoute(
        urls_to_cache[i],
        new workbox.strategies.StaleWhileRevalidate(),
    );
}
