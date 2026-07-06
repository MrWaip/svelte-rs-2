import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { position = "static", pb = "" } = $$props;
	$$renderer.push(`<div${$.attr_style(`position: ${$.stringify(position)}`, { "--x": pb })}></div>`);
}
