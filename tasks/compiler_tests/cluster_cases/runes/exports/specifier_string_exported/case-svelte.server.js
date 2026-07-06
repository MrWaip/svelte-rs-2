import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = 1;
	$$renderer.push(`<p>${$.escape(foo)}</p>`);
	$.bind_props($$props, { "foo-bar": foo });
}
