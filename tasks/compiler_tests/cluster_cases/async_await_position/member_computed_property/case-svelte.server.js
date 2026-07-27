import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	const list = [
		1,
		2,
		3
	];
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.push(async () => $.escape(list[(await $.save(delay(x)))()]));
}
