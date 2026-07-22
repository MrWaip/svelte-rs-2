import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div><p dir="rtl">static parent, no runtime reset</p></div>`);
}
