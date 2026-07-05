import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = [[1, 2], [3, 4]], $$array = $.to_array(tmp, 2), $$array_1 = $.to_array($$array[0], 2), $$array_2 = $.to_array($$array[1], 2), a = $$array_1[0], b = $$array_1[1], c = $$array_2[0], d = $$array_2[1];
	function bump() {
		a = a;
		b = b;
		c = c;
		d = d;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
}
