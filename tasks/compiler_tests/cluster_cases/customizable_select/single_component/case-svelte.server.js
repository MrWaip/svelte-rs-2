import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	$$renderer.push(`<select>`);
	Inner($$renderer, {});
	$$renderer.push(`<!----><!></select>`);
}
