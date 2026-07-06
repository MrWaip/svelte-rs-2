import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = {
		x: "a",
		z: ["b"]
	}, $$array = $.to_array(tmp.z, 1), foo = $.fallback($$props["foo"], () => $.fallback(tmp.x, "default-x"), true), bar = $.fallback($$props["bar"], () => $$array[0], true);
	$$renderer.push(`<p>${$.escape(foo)}${$.escape(bar)}</p>`);
	$.bind_props($$props, {
		foo,
		bar
	});
}
