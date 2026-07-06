App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { LINKS } from "./links.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let heading;
		let title = $.fallback($$props["title"], "");
		$: heading = title.toUpperCase();
		$$renderer.push(`<h1>`);
		$.push_element($$renderer, "h1", 8, 0);
		$$renderer.push(`${$.escape(heading)}</h1>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		const each_array = $.ensure_array_like(LINKS.list);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let link = each_array[$$index];
			$$renderer.push(`<a${$.attr("href", link.href)}>`);
			$.push_element($$renderer, "a", 10, 4);
			$$renderer.push(`${$.escape(link.label)}</a>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { title });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
