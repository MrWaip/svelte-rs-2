import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import { fade } from "svelte/transition";
export default function App($$renderer) {
	var data, params;
	var $$promises = $$renderer.run([async () => data = await fetch("/api"), () => params = data.params]);
	if (true) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div>hello</div>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
