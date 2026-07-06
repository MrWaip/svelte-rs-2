import * as $ from "svelte/internal/server";
const foo = "foo";
function bar() {
	return foo.toUpperCase();
}
export { foo, bar };
export default function App($$renderer) {
	$$renderer.push(`<p>module exports</p>`);
}
