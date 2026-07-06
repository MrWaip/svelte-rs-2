import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function load() {
		return 1;
	}
	const a = $.derived(load);
	const x = $.derived(() => a() + 1);
	$$renderer.push(`<h1>${$.escape(x())}</h1>`);
}
