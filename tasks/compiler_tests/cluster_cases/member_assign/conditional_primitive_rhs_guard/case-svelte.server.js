import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let result = {};
	let data = {};
	function fill(keys) {
		keys.forEach((key) => result[key] = data[key] ? true : false);
	}
	$$renderer.push(`<button>go</button>`);
}
