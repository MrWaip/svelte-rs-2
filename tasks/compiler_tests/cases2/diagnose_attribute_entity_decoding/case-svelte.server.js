import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div title="a b&amp;c&lt;d">x</div> `);
	Child($$renderer, { label: "a\xA0b&c<d" });
	$$renderer.push(`<!---->`);
}
