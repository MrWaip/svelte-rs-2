import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let s = $$props["s"];
	let attributes = $.fallback($$props["attributes"], () => ({}), true);
	$$renderer.push(`<div${$.attributes({
		style: s,
		...attributes
	})}></div>`);
	$.bind_props($$props, {
		s,
		attributes
	});
}
