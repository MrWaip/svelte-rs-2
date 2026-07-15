App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Svg from "./Svg.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let paths = [];
		let polygons = [];
		Svg($$renderer, {
			children: $.prevent_snippet_stringification(($$renderer) => {
				$$renderer.push(`<!--[-->`);
				$.slot($$renderer, $$props, "default", {}, () => {
					$$renderer.push(`<!--[-->`);
					const each_array = $.ensure_array_like(paths);
					for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
						let path = each_array[$$index];
						$$renderer.push(`<path${$.attributes({ ...path }, void 0, void 0, void 0, 3)}>`);
						$.push_element($$renderer, "path", 11, 3);
						$$renderer.push(`</path>`);
						$.pop_element();
					}
					$$renderer.push(`<!--]--><!--[-->`);
					const each_array_1 = $.ensure_array_like(polygons);
					for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
						let polygon = each_array_1[$$index_1];
						$$renderer.push(`<polygon${$.attributes({ ...polygon }, void 0, void 0, void 0, 3)}>`);
						$.push_element($$renderer, "polygon", 14, 3);
						$$renderer.push(`</polygon>`);
						$.pop_element();
					}
					$$renderer.push(`<!--]-->`);
				});
				$$renderer.push(`<!--]-->`);
			}),
			$$slots: { default: true }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
