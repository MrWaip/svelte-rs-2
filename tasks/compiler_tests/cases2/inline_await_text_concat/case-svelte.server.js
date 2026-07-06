import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var response;
	var $$promises = $$renderer.run([async () => response = await fetch("/api")]);
	$$renderer.push(`<p>Hello `);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(async () => $.escape((await $.save(response.text()))())));
	$$renderer.push(`!</p>`);
}
