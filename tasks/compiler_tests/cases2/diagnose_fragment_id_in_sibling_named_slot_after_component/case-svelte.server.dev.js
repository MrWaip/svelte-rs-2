App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
import Img from "./Img.svelte";
import Btn from "./Btn.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let cond = true;
		Outer($$renderer, { $$slots: {
			image: ($$renderer) => {
				Img($$renderer, { slot: "image" });
			},
			action: ($$renderer) => {
				Btn($$renderer, {
					slot: "action",
					children: $.prevent_snippet_stringification(($$renderer) => {
						if (cond) {
							$$renderer.push("<!--[0-->");
							$$renderer.push(`<span>`);
							$.push_element($$renderer, "span", 11, 18);
							$$renderer.push(`a</span>`);
							$.pop_element();
						} else {
							$$renderer.push("<!--[-1-->");
							$$renderer.push(`b`);
						}
						$$renderer.push(`<!--]-->`);
					}),
					$$slots: { default: true }
				});
			}
		} });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
