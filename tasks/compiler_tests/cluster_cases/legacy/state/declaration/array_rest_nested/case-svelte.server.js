import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = [
		1,
		2,
		3
	], $$array = $.to_array(tmp), $$array_1 = $.to_array($$array.slice(1), 2), a = $$array[0], b = $$array_1[0], c = $$array_1[1];
	function bump() {
		a = a;
		b = b;
		c = c;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}</button>`);
}
