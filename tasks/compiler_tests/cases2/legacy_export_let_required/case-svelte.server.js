import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	let baz = $.fallback($$props["baz"], undefined);
	$$renderer.push(`<p>${$.escape(foo)}${$.escape(baz)}</p>`);
	$.bind_props($$props, {
		foo,
		baz
	});
}
