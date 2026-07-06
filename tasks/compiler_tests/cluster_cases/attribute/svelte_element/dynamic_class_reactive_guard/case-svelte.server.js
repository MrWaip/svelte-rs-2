import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let flag = false;
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class("", void 0, { "blue": flag })}${$.attr_style("", { color: flag ? "red" : "blue" })}`);
	});
	$$renderer.push(` <button>toggle</button>`);
}
