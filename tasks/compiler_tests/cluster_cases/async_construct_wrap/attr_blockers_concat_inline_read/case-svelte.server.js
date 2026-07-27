import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function compute() {
		return "x";
	}
	var a;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => a = compute()]);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.push(`<div${$.attr_style(`a: ${$.stringify(a)}`, { width: a })}></div>`);
	});
}
