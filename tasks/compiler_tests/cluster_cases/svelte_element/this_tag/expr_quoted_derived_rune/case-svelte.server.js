import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	let tag = $.derived(() => "h" + n);
	function bump() {
		n++;
	}
	$$renderer.push(`<button>go</button> `);
	$.element($$renderer, tag(), void 0, () => {
		$$renderer.push(`hello`);
	});
}
