App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Foo($$renderer, {
			children: $.invalid_default_snippet,
			$$slots: { default: ($$renderer, { onClick }) => {
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 6, 1);
				$$renderer.push(`${$.escape(onClick)}</p>`);
				$.pop_element();
			} }
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
