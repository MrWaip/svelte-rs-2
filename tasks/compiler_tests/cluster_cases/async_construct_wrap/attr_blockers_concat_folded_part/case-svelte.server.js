import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var a, b, c;
	var $$promises = $$renderer.run([
		() => Promise.resolve(),
		() => a = "a",
		() => Promise.resolve(),
		() => b = "b",
		() => Promise.resolve(),
		() => c = "c"
	]);
	$$renderer.async([$$promises[5], $$promises[3]], ($$renderer) => {
		$$renderer.push(`<div${$.attr_style("w: a", { color: c })}${$.attr("title", b)}></div>`);
	});
}
