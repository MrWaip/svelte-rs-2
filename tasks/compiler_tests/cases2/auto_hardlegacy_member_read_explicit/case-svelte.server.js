import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { x: 0 };
	function bump() {
		obj.x += 1;
	}
	$$renderer.push(`<button>${$.escape(obj.x + 1)}</button>`);
}
