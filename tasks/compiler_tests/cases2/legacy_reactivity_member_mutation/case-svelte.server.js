import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let object = { x: 0 };
	function bump() {
		object.x += 1;
	}
	$$renderer.push(`<button>value: ${$.escape(object.x)}</button>`);
}
