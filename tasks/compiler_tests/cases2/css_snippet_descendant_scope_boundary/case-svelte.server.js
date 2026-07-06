import * as $ from "svelte/internal/server";
function summary($$renderer) {
	$$renderer.push(`<section class="summary svelte-ic1cb7">summary</section>`);
}
export default function App($$renderer) {
	let active = true;
	$$renderer.push(`<div${$.attr_class("chunk-shell svelte-ic1cb7", void 0, { "state": active })}>`);
	summary($$renderer);
	$$renderer.push(`<!----></div>`);
}
