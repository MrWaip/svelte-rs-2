import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div><my-element value="test"></my-element></div>`);
}
