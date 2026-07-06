import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let ref = void 0;
	function set(el) {
		ref = el;
	}
	function get() {
		return ref;
	}
	$$renderer.push(`<div></div>`);
}
