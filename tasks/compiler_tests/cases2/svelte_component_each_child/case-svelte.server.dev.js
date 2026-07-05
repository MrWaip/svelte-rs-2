App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import A from "./A.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let current = A;
		let items = [
			1,
			2,
			3
		];
		if (current) {
			$$renderer.push("<!--[-->");
			current($$renderer, {
				children: $.prevent_snippet_stringification(($$renderer) => {
					$$renderer.push(`<!--[-->`);
					const each_array = $.ensure_array_like(items);
					for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
						let item = each_array[$$index];
						$$renderer.push(`<span>`);
						$.push_element($$renderer, "span", 10, 8);
						$$renderer.push(`${$.escape(item)}</span>`);
						$.pop_element();
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
