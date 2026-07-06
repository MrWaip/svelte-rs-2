import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = 0;
	function make() {
		return {};
	}
	function reset() {
		value = make();
	}
	$$renderer.push(`<button>${$.escape(value)}</button>`);
}
