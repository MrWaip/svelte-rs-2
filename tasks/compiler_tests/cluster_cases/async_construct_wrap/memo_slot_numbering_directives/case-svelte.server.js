import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function fn() {
		return "red";
	}
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save(true))();
		$$renderer.push(`<div${$.attr_class("", void 0, { "a": $$0 })}${$.attr_style("", { color: fn() })}>x</div>`);
	});
}
