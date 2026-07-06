import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	let b = $$props["b"];
	$$renderer.push(`<input${$.attr("data-x", `${$.stringify(a)}${$.stringify(b)}`)}/>`);
	$.bind_props($$props, {
		a,
		b
	});
}
