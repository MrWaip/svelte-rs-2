import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let o = { tag: "div" };
	function bump() {
		o = { tag: "p" };
	}
	$$renderer.push(`<button>go</button> `);
	$.element($$renderer, o.tag, void 0, () => {
		$$renderer.push(`hello`);
	});
}
