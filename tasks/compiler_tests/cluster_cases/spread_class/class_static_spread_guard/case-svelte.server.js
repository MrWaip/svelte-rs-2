import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let attributes = $.fallback($$props["attributes"], () => ({}), true);
	$$renderer.push(`<div${$.attributes({
		class: "foo bar",
		...attributes
	})}></div>`);
	$.bind_props($$props, { attributes });
}
