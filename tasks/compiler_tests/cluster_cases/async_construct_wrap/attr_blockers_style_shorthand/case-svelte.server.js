import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var color, width;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => {
		color = "red";
		width = "1px";
	}]);
	$$renderer.push(`<div${$.attr_style("", { color })}></div> <div${$.attr_style("", { width })}></div>`);
}
