import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = 0;
	$$renderer.push(`<p>0</p>`);
}
