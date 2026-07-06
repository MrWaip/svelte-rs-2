import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	$$renderer.push(`<!---->${$.escape(a)}<button>inc</button>`);
}
