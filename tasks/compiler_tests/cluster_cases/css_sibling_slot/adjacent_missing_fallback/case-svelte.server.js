import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<x class="svelte-1schprl"></x> <!--[-->`);
	$.slot($$renderer, $$props, "default", {}, () => {
		$$renderer.push(`<y>fallback content</y>`);
	});
	$$renderer.push(`<!--]--> <z class="svelte-1schprl">this should be green if the slot fallback is not rendered</z>`);
}
