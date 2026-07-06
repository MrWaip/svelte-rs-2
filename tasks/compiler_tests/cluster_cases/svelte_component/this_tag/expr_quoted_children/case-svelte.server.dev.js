App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
import Bar from "./Bar.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = $$props["x"];
		if (x ? Foo : Bar) {
			$$renderer.push("<!--[-->");
			(x ? Foo : Bar)($$renderer, {
				answer: 42,
				children: $.prevent_snippet_stringification(($$renderer) => {
					$$renderer.push(`<span>`);
					$.push_element($$renderer, "span", 7, 1);
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
		$.bind_props($$props, { x });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
