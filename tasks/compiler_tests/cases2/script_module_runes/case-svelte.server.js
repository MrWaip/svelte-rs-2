import * as $ from "svelte/internal/server";
let shared = 0;
let doubled = $.derived(() => shared * 2);
export default function App($$renderer) {
	function increment() {
		shared++;
	}
	$$renderer.push(`<button>${$.escape(doubled())}</button>`);
}
