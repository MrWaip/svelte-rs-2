App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import A from "./A.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let current = A;
		if (current) {
			$$renderer.push("<!--[-->");
			current($$renderer, {
				answer: 42,
				children: $.prevent_snippet_stringification(($$renderer) => {
					$$renderer.push(`<span>`);
					$.push_element($$renderer, "span", 8, 4);
					$$renderer.push(`child</span>`);
					$.pop_element();
				}),
				$$slots: { default: true }
			});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
