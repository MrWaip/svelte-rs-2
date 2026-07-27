import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function read() {
		return count;
	}
	$$renderer.push(`<button>${$.escape(read())}</button>`);
}
