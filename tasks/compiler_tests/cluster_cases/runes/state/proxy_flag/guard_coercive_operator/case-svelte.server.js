import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = 0;
	function inc() {
		value += 1;
	}
	$$renderer.push(`<button>${$.escape(value)}</button>`);
}
