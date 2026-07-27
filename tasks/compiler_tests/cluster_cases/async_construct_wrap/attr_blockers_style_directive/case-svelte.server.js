import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var color, width;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => {
		color = "red";
		width = "1px";
	}]);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.push(`<div${$.attr_style("", { color })}></div>`);
	});
	$$renderer.push(` `);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.push(`<div${$.attr_style("", { width })}></div>`);
	});
}
