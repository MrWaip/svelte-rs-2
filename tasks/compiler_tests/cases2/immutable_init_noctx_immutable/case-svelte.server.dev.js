App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = $.fallback($$props["count"], 0);
		let foo = $.fallback($$props["foo"], () => ({ bar: "baz" }), true);
		$: if (foo) count += 1;
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 9, 0);
		$$renderer.push(`<h3>`);
		$.push_element($$renderer, "h3", 10, 1);
		$$renderer.push(`Called ${$.escape(count)} times.</h3>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, {
			count,
			foo
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
