import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a, b } = $$props;
	let eq = a != b;
	function toggle() {
		eq = !eq;
	}
	$$renderer.push(`<button>${$.escape(eq)}</button>`);
}
