import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "red";
	const getClass = () => value === "blue";
	const getValue = () => value;
	$$renderer.push(`<div${$.attr_class("", void 0, { "blue": getClass() })}${$.attr_style("", { color: getValue() })}></div> <button>toggle</button>`);
}
