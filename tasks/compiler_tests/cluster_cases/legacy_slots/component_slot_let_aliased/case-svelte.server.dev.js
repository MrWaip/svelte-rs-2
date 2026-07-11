App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Nested from "./Nested.svelte";
import SlotInner from "./SlotInner.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Nested($$renderer, { $$slots: { foo: ($$renderer, { thing: data }) => {
			SlotInner($$renderer, {
				slot: "foo",
				thing: data,
				children: $.prevent_snippet_stringification(($$renderer) => {
					$$renderer.push(`<div class="inner-slot">`);
					$.push_element($$renderer, "div", 8, 2);
					$$renderer.push(`${$.escape(data)}</div>`);
					$.pop_element();
				}),
				$$slots: { default: true }
			});
		} } });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
