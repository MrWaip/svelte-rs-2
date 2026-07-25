import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let size = 1;
	function bump() {
		size++;
	}
	$$renderer.push(`<button>go</button> `);
	$.element($$renderer, `h${size}`, void 0, () => {
		$$renderer.push(`hello`);
	});
}
