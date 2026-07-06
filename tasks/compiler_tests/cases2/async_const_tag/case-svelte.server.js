import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var data;
	var $$promises = $$renderer.run([async () => data = await fetch("/api")]);
	if (true) {
		$$renderer.push("<!--[0-->");
		let value;
		var promises = $$renderer.run([() => $$promises[0], () => value = data.text]);
		$$renderer.push(`<p>`);
		$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(value)));
		$$renderer.push(`</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
