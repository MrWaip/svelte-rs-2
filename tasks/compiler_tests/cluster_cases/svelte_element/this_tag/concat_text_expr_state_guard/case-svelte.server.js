import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 1;
	function bump() {
		n++;
	}
	$$renderer.push(`<button>go</button> `);
	$.element($$renderer, "h", void 0, () => {
		$$renderer.push(`hello`);
	});
}
