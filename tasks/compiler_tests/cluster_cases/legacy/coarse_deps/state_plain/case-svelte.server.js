import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 1;
	function inc() {
		count = count + 1;
	}
	$$renderer.push(`<button>+</button> `);
	if (count) {
		$$renderer.push("<!--[0-->");
		const label = count.toFixed(2);
		$$renderer.push(`<span>${$.escape(label)}</span>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
