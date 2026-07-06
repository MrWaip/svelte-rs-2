import * as $ from "svelte/internal/server";
function foo($$renderer, a, b) {
	$$renderer.push(`<!---->Hello world ${$.escape(a + b)}`);
}
export default function App($$renderer) {
	foo($$renderer, 1, 2);
}
