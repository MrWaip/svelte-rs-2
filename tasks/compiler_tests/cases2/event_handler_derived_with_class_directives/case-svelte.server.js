import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let counter = 0;
	let color = "red";
	function getHandler() {
		return () => {
			counter++;
			color = "blue";
		};
	}
	$$renderer.push(`<div${$.attr_class("", void 0, {
		"active": counter > 5,
		"big": counter > 10
	})}${$.attr_style("", { color })}>content</div>`);
}
