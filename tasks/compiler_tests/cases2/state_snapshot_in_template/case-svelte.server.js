import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [
		1,
		2,
		3
	];
	$$renderer.push(`<p>${$.escape(JSON.stringify($.snapshot(items)))}</p>`);
}
