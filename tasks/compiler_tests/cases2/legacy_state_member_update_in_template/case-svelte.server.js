import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { x: 0 };
	$$renderer.push(`<!---->${$.escape(obj.x++)}`);
}
