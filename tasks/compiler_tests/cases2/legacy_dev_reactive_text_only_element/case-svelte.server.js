import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let title = "x";
	let count = 0;
	function tick() {
		title = "y";
		count = 1;
	}
	$$renderer.push(`<p>${$.escape(title)}</p> <strong>${$.escape(count)}</strong> `);
	$.element($$renderer, "div", void 0, () => {
		$$renderer.push(`Dyn: ${$.escape(title)}`);
	});
}
