import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let value = $$props["value"];
	const onClick = (v) => () => {
		value = v;
	};
	$$renderer.push(`<div${$.attr_class("chip", void 0, { "active": value === 1 })}>hi</div>`);
	$.bind_props($$props, { value });
}
