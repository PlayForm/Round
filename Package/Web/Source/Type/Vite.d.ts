/// <reference types="vite/client" />

// `import.meta.env` is Vite's compile-time environment, and its types live in
// `vite/client` - which nothing in this package referenced, so `import.meta.env`
// failed with TS2339 the first time it was used. This file is the reference; it
// declares nothing itself.
