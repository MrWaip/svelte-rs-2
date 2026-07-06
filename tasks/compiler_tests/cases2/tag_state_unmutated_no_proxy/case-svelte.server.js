import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let num = 42;
	$$renderer.push(`<p>42</p>`);
}
