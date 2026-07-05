import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { id = "" } = $$props;
	function getStyle() {
		return "color:red;";
	}
	$$renderer.push(`<div${$.attr("data-testid", id)}${$.attr_style(getStyle())}></div>`);
}
