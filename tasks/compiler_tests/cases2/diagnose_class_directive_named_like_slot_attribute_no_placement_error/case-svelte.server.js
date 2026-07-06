import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { cond = false } = $$props;
	$$renderer.push(`<div${$.attr_class("header", void 0, { "slot": cond })}>hi</div>`);
}
