import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var a;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => a = 0]);
	$$renderer.push(`<div></div>`);
}
