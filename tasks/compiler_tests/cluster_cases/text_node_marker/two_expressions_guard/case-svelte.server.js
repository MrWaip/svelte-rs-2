import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	let b = 0;
	$$renderer.push(`<!---->${$.escape(a)}${$.escape(b)}<button>inc</button>`);
}
