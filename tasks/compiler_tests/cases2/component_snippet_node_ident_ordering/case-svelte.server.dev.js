App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { Layout, Btn, Cap } = $$props;
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 5, 0);
		{
			$.prevent_snippet_stringification(footer);
			function footer($$renderer) {
				$.validate_snippet_args($$renderer);
				if (Btn) {
					$$renderer.push("<!--[-->");
					Btn($$renderer, {});
					$$renderer.push("<!--]-->");
				} else {
					$$renderer.push("<!--[!-->");
					$$renderer.push("<!--]-->");
				}
				$$renderer.push(` <div class="cap">`);
				$.push_element($$renderer, "div", 9, 12);
				if (Cap) {
					$$renderer.push("<!--[-->");
					Cap($$renderer, {});
					$$renderer.push("<!--]-->");
				} else {
					$$renderer.push("<!--[!-->");
					$$renderer.push("<!--]-->");
				}
				$$renderer.push(`</div>`);
				$.pop_element();
			}
			if (Layout) {
				$$renderer.push("<!--[-->");
				Layout($$renderer, {
					footer,
					$$slots: { footer: true }
				});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		}
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
