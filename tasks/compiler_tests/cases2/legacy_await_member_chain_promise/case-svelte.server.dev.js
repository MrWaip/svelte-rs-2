App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let user = $.fallback($$props["user"], () => ({ fetch: () => Promise.resolve(null) }), true);
		$.await($$renderer, user.fetch(), () => {}, (v) => {
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 7, 4);
			$$renderer.push(`${$.escape(v)}</span>`);
			$.pop_element();
		});
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { user });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
