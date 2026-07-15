import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let b = $$props["b"];
	let attributes = $.fallback($$props["attributes"], () => ({}), true);
	$$renderer.push(`<div${$.attributes({
		class: `a ${$.stringify(b)}`,
		...attributes
	})}></div>`);
	$.bind_props($$props, {
		b,
		attributes
	});
}
