import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<div class="icon-slot svelte-1qciquw"><!--[-->`);
	$.slot($$renderer, $$props, "icon", {}, () => {
		$$renderer.push(`<img alt="" class="svelte-1qciquw"/>`);
	});
	$$renderer.push(`<!--]--></div>`);
}
