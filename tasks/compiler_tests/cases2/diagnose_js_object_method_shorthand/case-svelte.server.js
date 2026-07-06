import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const obj = { run(x) {
		return x + 1;
	} };
	$$renderer.push(`<p>${$.escape(obj.run(1))}</p>`);
}
