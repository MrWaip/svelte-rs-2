import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function inc() {
		count += 1;
	}
	$$renderer.push(`<button>${$.escape(count)}</button>`);
}
