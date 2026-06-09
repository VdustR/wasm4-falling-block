const CACHE_NAME = "falling-block-__CACHE_VERSION__";
const CACHE_ASSETS = [
  "./index.html",
  "./cart.html",
  "./manifest.webmanifest",
  "./favicon.ico",
  "./icon.svg",
  "./icon-192.png",
  "./icon-512.png",
  "./main-visual.svg",
  "./main-visual.png",
  "./og-image.png",
  "./prototype.png"
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(CACHE_ASSETS))
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(keys.filter((key) => key !== CACHE_NAME).map((key) => caches.delete(key)))
      )
      .then(() => self.clients.claim())
  );
});

self.addEventListener("message", (event) => {
  if (event.data?.type === "SKIP_WAITING") {
    self.skipWaiting();
  }
});

self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET") {
    return;
  }

  if (event.request.mode === "navigate") {
    const url = new URL(event.request.url);
    const fallback = url.pathname.endsWith("/cart.html") ? "./cart.html" : "./index.html";
    event.respondWith(
      fetch(event.request)
        .then((response) => {
          if (!response.ok) {
            return response;
          }

          const copy = response.clone();
          event.waitUntil(caches.open(CACHE_NAME).then((cache) => cache.put(fallback, copy)));
          return response;
        })
        .catch(() => caches.match(fallback))
    );
    return;
  }

  event.respondWith(
    caches.match(event.request).then((cached) => {
      if (cached) {
        return cached;
      }

      return fetch(event.request)
        .then((response) => {
          const copy = response.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(event.request, copy));
          return response;
        })
        .catch(() => caches.match("./index.html"));
    })
  );
});
