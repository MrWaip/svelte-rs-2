import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var foo = $.fallback($$props["foo"], 1);
	function getFoo() {
		return foo;
	}
	$$renderer.push(`<p>${$.escape(foo)}</p>`);
	$.bind_props($$props, {
		foo,
		getFoo
	});
}
