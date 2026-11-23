// `tauri::generate_context!` embeds the frontend into the binary at compile
// time from `Crate/Round/tauri.conf.json` -> `frontendDist: "Web"`. That path
// has to live INSIDE the crate, or `cargo package` cannot ship it and a
// published `pround` would need a Node toolchain to install.
//
// Vite's canonical output stays `Package/Web/Target`; this step mirrors it into
// the crate so both statements are true at once.
import { cpSync, existsSync, rmSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const from = join(here, "..", "Target");
const to = join(here, "..", "..", "..", "Crate", "Round", "Web");

if (!existsSync(from)) {
	console.error(`[Embed] no build output at ${from} - run the vite build first.`);
	process.exit(1);
}

rmSync(to, { recursive: true, force: true });
cpSync(from, to, { recursive: true });

console.log(`[Embed] ${relative(process.cwd(), from)} -> ${relative(process.cwd(), to)}`);
