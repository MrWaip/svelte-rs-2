import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "a";
	function toggle() {
		value = value === "a" ? "b" : "c";
	}
	$$renderer.push(`<button>${$.escape(value)}</button>`);
}
