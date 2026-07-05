import * as $ from "svelte/internal/server";
import data from "./dep.js";
export default function App($$renderer) {
	let doubled;
	let count = 0;
	$: doubled = { value: count };
	$$renderer.push(`<!---->${$.escape(doubled.value)}`);
}
