import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
function content($$renderer, value) {
	$$renderer.push(`<p>${$.escape(value)}</p>`);
}
export default function App($$renderer) {
	var data;
	var $$promises = $$renderer.run([async () => data = await fetch("/api")]);
	$$renderer.async_block([$$promises[0]], ($$renderer) => {
		content($$renderer, data);
	});
}
