import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = {
		a: 1,
		b: 2,
		c: 3
	}, a = tmp.a, rest = $.exclude_from_object(tmp, ["a"]);
	function bump() {
		a = a;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.b)}</button>`);
}
