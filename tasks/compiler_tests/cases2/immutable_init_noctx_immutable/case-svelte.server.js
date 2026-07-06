import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = $.fallback($$props["count"], 0);
	let foo = $.fallback($$props["foo"], () => ({ bar: "baz" }), true);
	$: if (foo) count += 1;
	$$renderer.push(`<div><h3>Called ${$.escape(count)} times.</h3></div>`);
	$.bind_props($$props, {
		count,
		foo
	});
}
