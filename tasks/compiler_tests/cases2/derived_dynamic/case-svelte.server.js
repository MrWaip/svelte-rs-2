import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	let doubled = $.derived(() => count * 2);
	function increment() {
		count++;
	}
	$$renderer.push(`<button>${$.escape(doubled())}</button>`);
}
