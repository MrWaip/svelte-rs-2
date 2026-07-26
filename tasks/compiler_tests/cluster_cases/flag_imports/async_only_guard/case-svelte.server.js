import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	$$renderer.push(`<button>${$.escape(count)}</button>`);
}
