import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = [
		1,
		2,
		3
	], $$array = $.to_array(tmp), a = $$array[0], rest = $$array.slice(1);
	function bump() {
		a = a;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.length)}</button>`);
}
