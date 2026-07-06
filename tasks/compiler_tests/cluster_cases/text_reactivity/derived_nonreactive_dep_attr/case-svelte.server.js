import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function load() {
		return { foo: 1 };
	}
	const c = load();
	const x = $.derived(() => c.foo);
	$$renderer.push(`<div${$.attr("title", x())}></div>`);
}
