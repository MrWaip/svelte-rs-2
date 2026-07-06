import * as $ from "svelte/internal/server";
function funBind($$renderer, context) {
	$$renderer.push(`<input/>`);
}
export default function App($$renderer) {
	funBind($$renderer, { set element(e) {} });
}
