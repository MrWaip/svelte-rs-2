App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.push(`<meta name="description" content="A"/>`);
			$.push_element($$renderer, "meta", 6, 1);
			$.pop_element();
		});
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 9, 0);
		$$renderer.push(`x</span>`);
		$.pop_element();
		$$renderer.push(` ${$.escape(foo)}`);
		$.bind_props($$props, { foo });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
