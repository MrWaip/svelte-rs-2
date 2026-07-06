import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let active = true;
	$$renderer.push(`<div${$.attr_class("", void 0, { "active": active })}></div>`);
}
