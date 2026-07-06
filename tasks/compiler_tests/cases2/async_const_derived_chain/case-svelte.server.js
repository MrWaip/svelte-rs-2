import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var data;
	var $$promises = $$renderer.run([async () => data = await fetch("/api")]);
	if (true) {
		$$renderer.push("<!--[0-->");
		let a;
		let b;
		let c;
		var promises = $$renderer.run([
			() => $$promises[0],
			() => a = data.value,
			() => b = a * 2,
			() => c = b + 1
		]);
		$$renderer.push(`<p>`);
		$$renderer.async([promises[3]], ($$renderer) => $$renderer.push(() => $.escape(c)));
		$$renderer.push(`</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
