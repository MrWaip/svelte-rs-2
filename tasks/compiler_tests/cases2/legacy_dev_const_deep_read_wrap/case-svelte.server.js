import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let state = "";
	function bump() {
		state = state + "x";
	}
	$$renderer.push(`<button>bump</button> `);
	if (state) {
		$$renderer.push("<!--[0-->");
		const localLen = state.length;
		$$renderer.push(`<span>Length: ${$.escape(localLen)}</span>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
