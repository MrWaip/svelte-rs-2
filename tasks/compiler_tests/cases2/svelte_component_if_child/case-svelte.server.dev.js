App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import A from "./A.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let current = A;
		let cond = false;
		if (current) {
			$$renderer.push("<!--[-->");
			current($$renderer, {
				children: $.prevent_snippet_stringification(($$renderer) => {
					if (cond) {
						$$renderer.push("<!--[0-->");
						$$renderer.push(`<span>`);
						$.push_element($$renderer, "span", 10, 8);
						$$renderer.push(`child</span>`);
						$.pop_element();
					} else {
						$$renderer.push("<!--[-1-->");
					}
					$$renderer.push(`<!--]-->`);
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
