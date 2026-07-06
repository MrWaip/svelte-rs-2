import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let value = "red";
	const getClass = () => value === "blue";
	const getValue = () => value;
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class("", void 0, { "blue": getClass() })}${$.attr_style("", { color: getValue() })}`);
	});
	$$renderer.push(` <button>toggle</button>`);
}
