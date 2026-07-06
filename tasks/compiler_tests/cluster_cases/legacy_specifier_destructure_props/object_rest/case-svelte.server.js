import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = {
		a: 1,
		b: 2,
		c: 3
	}, a = $.fallback($$props["a"], () => tmp.a, true), rest = $.fallback($$props["rest"], () => $.exclude_from_object(tmp, ["a"]), true);
	function inc() {
		a++;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(JSON.stringify(rest))}</button>`);
	$.bind_props($$props, { a });
}
