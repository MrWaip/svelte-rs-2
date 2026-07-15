import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let myClass = $$props["myClass"];
	let flag = $$props["flag"];
	let attributes = $.fallback($$props["attributes"], () => ({}), true);
	$$renderer.push(`<div${$.attributes({
		class: $.clsx(myClass),
		...attributes
	}, void 0, { on: flag })}></div>`);
	$.bind_props($$props, {
		myClass,
		flag,
		attributes
	});
}
