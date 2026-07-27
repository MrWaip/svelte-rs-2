import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	function bump() {
		tag = "p";
	}
	$$renderer.push(`<button>go</button> `);
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`hello`);
	});
}
