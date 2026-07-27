import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function bump() {
		count += 1;
	}
	$$renderer.push(`<div${$.attr("title", [(x) => x])}></div> <button>go</button>`);
}
