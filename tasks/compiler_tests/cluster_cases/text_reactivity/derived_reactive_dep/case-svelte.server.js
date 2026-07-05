import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let s = 0;
	function inc() {
		s++;
	}
	const x = $.derived(() => s + 1);
	$$renderer.push(`<h1>${$.escape(x())}</h1>`);
}
