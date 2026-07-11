App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	$$renderer.component(($$renderer) => {
		const { $$slots: $$slots_, $$events, ...props } = $$props;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 5, 0);
		$$renderer.push(`${$.escape(Object.keys(props))}</p>`);
		$.pop_element();
		$$renderer.push(` `);
		if ($$slots.foo) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 8, 1);
			$$renderer.push(`foo exists</p>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
