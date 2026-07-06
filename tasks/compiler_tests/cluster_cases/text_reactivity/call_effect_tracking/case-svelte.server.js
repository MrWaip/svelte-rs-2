import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<p>v ${$.escape(false)}</p>`);
}
