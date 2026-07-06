import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { style = "", size = "s", label = "" } = $$props;
	$$renderer.push(`<div${$.attr_style(style, { width: "100px" })}${$.attr_class(`box ${$.stringify(size)}`)}${$.attr("data-label", label)}></div>`);
}
