import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let x = $$props["x"];
	$$renderer.push(`<div${$.attr_class("", void 0, { "before-content": $$slots.beforeContent })}><!--[-->`);
	$.slot($$renderer, $$props, "beforeContent", {}, null);
	$$renderer.push(`<!--]--> ${$.escape(x)}</div>`);
	$.bind_props($$props, { x });
}
