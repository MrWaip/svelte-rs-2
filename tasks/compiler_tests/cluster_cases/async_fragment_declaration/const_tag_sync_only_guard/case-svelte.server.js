import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let source = {
		x: 1,
		y: 2
	};
	if (source) {
		$$renderer.push("<!--[0-->");
		const a = source.x;
		const { x, y } = source;
		$$renderer.push(`<p>${$.escape(a)}${$.escape(x)}${$.escape(y)}</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <button>go</button>`);
}
