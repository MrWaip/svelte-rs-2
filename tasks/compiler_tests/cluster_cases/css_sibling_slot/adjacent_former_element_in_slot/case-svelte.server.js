import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "default", {}, () => {
		$$renderer.push(`<h1 class="svelte-142nm4m">test</h1>`);
	});
	$$renderer.push(`<!--]--> <span class="svelte-142nm4m">Hello</span>`);
}
