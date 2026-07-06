App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const { x } = $$props;
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 5, 0);
		Cmp($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				if (x) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<span>`);
					$.push_element($$renderer, "span", 8, 12);
					$$renderer.push(`a</span>`);
					$.pop_element();
				} else {
					$$renderer.push("<!--[-1-->");
				}
				$$renderer.push(`<!--]--> <p>`);
				$.push_element($$renderer, "p", 10, 8);
				$$renderer.push(`tail</p>`);
				$.pop_element();
			}),
			$$slots: { default: true }
		});
		$$renderer.push(`<!----></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
