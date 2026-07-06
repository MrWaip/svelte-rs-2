import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $.fallback($$props["foo"], 1);
	function makeBar() {
		return 42;
	}
	let bar = $.fallback($$props["bar"], makeBar, true);
	$$renderer.push(`<p>${$.escape(foo)}${$.escape(bar)}</p>`);
	$.bind_props($$props, {
		foo,
		bar
	});
}
