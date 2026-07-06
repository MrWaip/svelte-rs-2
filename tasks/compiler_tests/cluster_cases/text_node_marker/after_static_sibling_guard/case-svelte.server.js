import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	$$renderer.push(`<br/>${$.escape(a)}<button>inc</button>`);
}
