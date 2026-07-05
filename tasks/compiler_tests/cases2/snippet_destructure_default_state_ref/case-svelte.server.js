import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let counter = 0;
	function row($$renderer, { values = [counter] }) {
		$$renderer.push(`<span>${$.escape(values.length)}</span>`);
	}
	row($$renderer, {});
}
