import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function getValue() {
		return loaded + value;
	}
	function setValue(v) {
		value = v;
	}
	var loaded, value;
	var $$promises = $$renderer.run([async () => loaded = await Promise.resolve(1), () => value = ""]);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.push(`<input${$.attr("value", getValue())}/>`);
	});
}
