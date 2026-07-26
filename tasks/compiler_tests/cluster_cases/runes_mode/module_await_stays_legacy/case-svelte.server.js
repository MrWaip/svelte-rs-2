import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
const shared = await Promise.resolve(1);
export default function App($$renderer) {
	let count = 0;
	$$renderer.push(`<p>${$.escape(shared)} ${$.escape(count)}</p> <button>inc</button>`);
}
