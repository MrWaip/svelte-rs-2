import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = {
		p: [1, 2],
		q: [3, 4]
	}, $$array = $.to_array(tmp.p, 2), $$array_1 = $.to_array(tmp.q, 2), a = $$array[0], b = $$array[1], c = $$array_1[0], d = $$array_1[1];
	function bump() {
		a = a;
		b = b;
		c = c;
		d = d;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
}
