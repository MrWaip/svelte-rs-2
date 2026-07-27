import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var a, b;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => {
		a = "a";
		b = "b";
	}]);
	$$renderer.async([$$promises[1], $$promises[1]], ($$renderer) => {
		$$renderer.push(`<div${$.attr_class("", void 0, {
			"one": a,
			"two": b
		})}></div>`);
	});
}
