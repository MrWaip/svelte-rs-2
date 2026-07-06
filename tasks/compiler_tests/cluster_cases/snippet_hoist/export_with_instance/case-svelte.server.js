import * as $ from "svelte/internal/server";
function foo($$renderer) {
	$$renderer.push(`<!---->oo`);
}
export { foo };
export default function App($$renderer) {
	let name = "world";
	$$renderer.push(`<h1>Hello world!</h1>`);
}
