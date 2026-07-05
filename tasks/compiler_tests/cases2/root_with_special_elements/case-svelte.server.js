import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function handleScroll() {
		count++;
	}
	$$renderer.push(`<div>Count: ${$.escape(count)}</div>`);
}
