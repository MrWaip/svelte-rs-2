import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var response;
	var $$promises = $$renderer.run([async () => response = await fetch("/api")]);
	$$renderer.async([$$promises[0]], async ($$renderer) => {
		const $$0 = (await $.save(response.text()))();
		$$renderer.push(`<div${$.attr("title", $$0)}></div>`);
	});
}
