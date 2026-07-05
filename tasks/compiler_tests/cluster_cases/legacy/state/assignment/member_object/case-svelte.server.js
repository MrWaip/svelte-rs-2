import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { x: 0 };
	let src = { v: 1 };
	function go() {
		({v: obj.x} = src);
	}
	$$renderer.push(`<button>${$.escape(obj.x)}</button>`);
}
