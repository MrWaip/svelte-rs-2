import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	let items = [
		1,
		2,
		3
	];
	let empty = void 0;
	let readonly_obj = { x: 1 };
	count = 10;
	count += 5;
	items = [
		4,
		5,
		6
	];
	$$renderer.push(`<div>${$.escape(count)}</div>`);
}
