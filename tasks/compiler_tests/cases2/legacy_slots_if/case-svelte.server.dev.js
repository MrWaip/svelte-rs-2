App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 1, 0);
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "title", {}, null);
		$$renderer.push(`<!--]--> `);
		if ($$slots.description) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<hr/>`);
			$.push_element($$renderer, "hr", 4, 2);
			$.pop_element();
			$$renderer.push(` <!--[-->`);
			$.slot($$renderer, $$props, "description", {}, null);
			$$renderer.push(`<!--]-->`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--></div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
