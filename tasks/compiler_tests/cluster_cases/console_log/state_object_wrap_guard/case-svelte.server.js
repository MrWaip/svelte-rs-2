import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { a: 1 };
	function report() {
		console.log(obj);
	}
	$$renderer.push(`<button>go</button>`);
}
