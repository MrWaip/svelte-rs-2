import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = { outer: [{ inner: 1 }] }, $$array = $.to_array(tmp.outer, 1), inner = $$array[0].inner;
	function bump() {
		inner = inner;
	}
	$$renderer.push(`<button>${$.escape(inner)}</button>`);
}
