import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div${$.attr_class("", void 0, { "on": String((y) => y) })}></div>`);
}
