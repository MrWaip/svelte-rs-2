import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let c = $$props["c"];
	let s = $$props["s"];
	let attributes = $.fallback($$props["attributes"], () => ({}), true);
	$$renderer.push(`<div${$.attributes({
		class: $.clsx(c),
		style: s,
		...attributes
	})}></div>`);
	$.bind_props($$props, {
		c,
		s,
		attributes
	});
}
