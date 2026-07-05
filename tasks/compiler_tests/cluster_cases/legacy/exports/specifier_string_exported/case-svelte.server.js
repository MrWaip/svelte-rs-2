import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = 1;
	$$renderer.push(`<p>1</p>`);
}
