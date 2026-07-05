import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = [1, 2], $$array = $.to_array(tmp, 2), a = $$array[0], b = $$array[1];
	function bump() {
		a = a;
		b = b;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
