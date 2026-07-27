import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pending = null;
	$$renderer.push(`<!--[!-->`);
	pending($$renderer);
	$$renderer.push(`<!--]-->`);
}
