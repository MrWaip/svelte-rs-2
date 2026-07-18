App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "default", {}, () => {
			$$renderer.push(`<!--[-->`);
			$.slot($$renderer, $$props, "default", {}, () => {
				$$renderer.push(`<h1 class="svelte-11j9i8n">`);
				$.push_element($$renderer, "h1", 3, 2);
				$$renderer.push(`Heading 1</h1>`);
				$.pop_element();
			});
			$$renderer.push(`<!--]-->`);
		});
		$$renderer.push(`<!--]--> <span>`);
		$.push_element($$renderer, "span", 6, 0);
		$$renderer.push(`Span 1</span>`);
		$.pop_element();
		$$renderer.push(` <span>`);
		$.push_element($$renderer, "span", 7, 0);
		$$renderer.push(`Span 2</span>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		$.slot($$renderer, $$props, "default", {}, () => {
			$$renderer.push(`<!--[-->`);
			$.slot($$renderer, $$props, "default", {}, () => {
				$$renderer.push(`<p class="svelte-11j9i8n">`);
				$.push_element($$renderer, "p", 10, 2);
				$$renderer.push(`Paragraph 2</p>`);
				$.pop_element();
			});
			$$renderer.push(`<!--]-->`);
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
