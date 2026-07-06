import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var value;
	var $$promises = $$renderer.run([async () => value = await fetch("/api")]);
	$$renderer.push(`<p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(async () => $.escape((await $.save(value))())));
	$$renderer.push(`</p>`);
}
