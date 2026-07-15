import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	let a = $$props["a"];
	let b = $$props["b"];
	let attributes = $.fallback($$props["attributes"], () => ({}), true);
	$$renderer.push(`<div${$.attributes({
		class: $.clsx(foo ? a : b),
		...attributes
	})}></div>`);
	$.bind_props($$props, {
		foo,
		a,
		b,
		attributes
	});
}
