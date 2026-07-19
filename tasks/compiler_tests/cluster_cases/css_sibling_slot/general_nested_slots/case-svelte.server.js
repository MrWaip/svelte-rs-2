import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "default", {}, () => {
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "default", {}, () => {
			$$renderer.push(`<h1 class="svelte-11j9i8n">Heading 1</h1>`);
		});
		$$renderer.push(`<!--]-->`);
	});
	$$renderer.push(`<!--]--> <span>Span 1</span> <span>Span 2</span> <!--[-->`);
	$.slot($$renderer, $$props, "default", {}, () => {
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "default", {}, () => {
			$$renderer.push(`<p class="svelte-11j9i8n">Paragraph 2</p>`);
		});
		$$renderer.push(`<!--]-->`);
	});
	$$renderer.push(`<!--]-->`);
}
