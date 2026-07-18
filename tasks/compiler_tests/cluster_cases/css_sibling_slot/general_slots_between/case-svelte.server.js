import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<h1 class="svelte-1j0crw7">Heading 1</h1> <!--[-->`);
	$.slot($$renderer, $$props, "default", {}, () => {
		$$renderer.push(`<span class="svelte-1j0crw7">Span 1</span>`);
	});
	$$renderer.push(`<!--]--> <!--[-->`);
	$.slot($$renderer, $$props, "default", {}, () => {
		$$renderer.push(`<span class="svelte-1j0crw7">Span 2</span>`);
	});
	$$renderer.push(`<!--]--> <p class="svelte-1j0crw7">Paragraph 2</p>`);
}
