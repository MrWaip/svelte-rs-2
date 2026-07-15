import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let myClass = $$props["myClass"];
	let attributes = $.fallback($$props["attributes"], () => ({}), true);
	$$renderer.push(`<div${$.attributes({
		class: $.clsx(myClass),
		...attributes
	})}></div>`);
	$.bind_props($$props, {
		myClass,
		attributes
	});
}
