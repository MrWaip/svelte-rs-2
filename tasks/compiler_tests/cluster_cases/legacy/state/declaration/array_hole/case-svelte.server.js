import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = [
		1,
		2,
		3
	], $$array = $.to_array(tmp, 3), a = $$array[0], c = $$array[2];
	function bump() {
		a = a;
		c = c;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(c)}</button>`);
}
