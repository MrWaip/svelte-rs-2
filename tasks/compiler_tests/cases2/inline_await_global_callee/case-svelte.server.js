import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let name = "world";
	$$renderer.push(`<h1>Hello world!</h1> `);
	$$renderer.push(async () => $.escape(await fetch()));
}
