import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let x = "x1";
	$$renderer.push(`<button>x</button> `);
	Child($$renderer, { x });
	$$renderer.push(`<!---->`);
}
