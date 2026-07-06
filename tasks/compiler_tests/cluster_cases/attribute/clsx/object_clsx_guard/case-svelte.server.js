import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let x = $$props["x"];
	$$renderer.push(`<div${$.attr_class($.clsx({ foo: x }))}></div>`);
	$.bind_props($$props, { x });
}
