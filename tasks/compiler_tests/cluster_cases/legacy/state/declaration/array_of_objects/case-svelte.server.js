import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = [{ a: 1 }, { b: 2 }], $$array = $.to_array(tmp, 2), a = $$array[0].a, b = $$array[1].b;
	function bump() {
		a = a;
		b = b;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
