import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "hi";
	$$renderer.push(`<pre>hi
</pre>`);
}
