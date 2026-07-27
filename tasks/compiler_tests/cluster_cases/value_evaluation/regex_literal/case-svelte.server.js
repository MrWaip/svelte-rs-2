import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const r = /ab/;
	$$renderer.push(`<p>/ab/ object x/ab/</p>`);
}
