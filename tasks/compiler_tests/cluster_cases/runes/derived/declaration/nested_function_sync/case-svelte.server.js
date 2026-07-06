import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function read() {
		let double = $.derived(() => count * 2);
		return double();
	}
	$$renderer.push(`<button>${$.escape(read())}</button>`);
}
