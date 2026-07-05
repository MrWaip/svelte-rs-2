import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const initial = 0;
	let value = initial;
	function bump() {
		value += 1;
	}
	$$renderer.push(`<button>${$.escape(value)}</button>`);
}
