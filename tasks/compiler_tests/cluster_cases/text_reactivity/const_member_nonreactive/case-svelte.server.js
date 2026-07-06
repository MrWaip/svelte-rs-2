import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function load() {
		return { foo: 1 };
	}
	const c = load();
	const x = c.foo;
	$$renderer.push(`<h1>${$.escape(x)}</h1>`);
}
