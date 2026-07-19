import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let v = "dog";
	$$renderer.push(`<select>`);
	$$renderer.option({}, v);
	$$renderer.push(`</select> <button>swap</button>`);
}
