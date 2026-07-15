import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let on = $$props["on"];
	let attributes = $.fallback($$props["attributes"], () => ({}), true);
	$$renderer.push(`<div${$.attributes({
		class: $.clsx({ active: on }),
		...attributes
	})}></div>`);
	$.bind_props($$props, {
		on,
		attributes
	});
}
