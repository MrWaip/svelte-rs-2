import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let c = $$props["c"];
	$$renderer.push(`<div${$.attr_class($.clsx(c))}></div>`);
	$.bind_props($$props, { c });
}
