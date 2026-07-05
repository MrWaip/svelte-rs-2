import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
function content($$renderer, value, extra) {
	$$renderer.push(`<p>${$.escape(value)}${$.escape(extra)}</p>`);
}
export default function App($$renderer) {
	var response;
	var $$promises = $$renderer.run([async () => response = await fetch("/api")]);
	$$renderer.async_block([$$promises[0]], async ($$renderer) => {
		const $$0 = (await $.save(response.text()))();
		content($$renderer, response, $$0);
	});
}
