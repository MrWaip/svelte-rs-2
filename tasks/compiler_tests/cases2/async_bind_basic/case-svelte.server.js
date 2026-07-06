import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 1;
	var data, value;
	var $$promises = $$renderer.run([async () => data = await fetch("/api"), () => value = data.text]);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.push(`<input${$.attr("value", value)}/>`);
	});
}
