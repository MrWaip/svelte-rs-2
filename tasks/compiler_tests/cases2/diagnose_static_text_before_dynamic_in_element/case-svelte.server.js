import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let name = "x";
	$$renderer.push(`<h3>Hello<br/>x</h3>`);
}
