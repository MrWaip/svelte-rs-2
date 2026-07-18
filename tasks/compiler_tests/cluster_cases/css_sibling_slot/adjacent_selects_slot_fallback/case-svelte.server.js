import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<h1 class="svelte-142nm4m">test</h1> <!--[-->`);
	$.slot($$renderer, $$props, "default", {}, () => {
		$$renderer.push(`<span class="svelte-142nm4m">Hello</span>`);
	});
	$$renderer.push(`<!--]-->`);
}
