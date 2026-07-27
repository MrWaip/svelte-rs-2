import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	let b = $$props["b"];
	$$renderer.push(`<div${$.attr("foo", `${$.stringify(a)}${$.stringify(b)}`)}></div>`);
	$.bind_props($$props, {
		a,
		b
	});
}
