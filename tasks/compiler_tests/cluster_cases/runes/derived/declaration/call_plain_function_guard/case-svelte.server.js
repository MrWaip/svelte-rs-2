import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function makeValue() {
		return 42;
	}
	const value = $.derived(makeValue);
	$$renderer.push(`<span>${$.escape(value())}</span>`);
}
