import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let target;
	$$renderer.push(`<div>x</div>`);
}
