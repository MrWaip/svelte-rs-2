import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let r = /ab/;
	$$renderer.push(`<button>x</button> <p>${$.escape(r)}</p>`);
}
