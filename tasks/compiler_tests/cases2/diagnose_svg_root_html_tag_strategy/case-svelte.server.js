import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let raw = "<g><circle r={10}/></g>";
	$$renderer.push(`${$.html(raw)}<g><path d="M1"></path></g>`);
}
