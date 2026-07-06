App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { page } from "$app/stores";
$.prevent_snippet_stringification(defaultWrapWith);
function defaultWrapWith($$renderer, mf) {
	$.validate_snippet_args($$renderer);
	mf($$renderer);
	$$renderer.push(`<!---->`);
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { wrapWith = defaultWrapWith } = $$props;
		$.prevent_snippet_stringification(mf);
		function mf($$renderer) {
			$.validate_snippet_args($$renderer);
			if ($.store_get($$store_subs ??= {}, "$page", page).url) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<b>`);
				$.push_element($$renderer, "b", 12, 8);
				$$renderer.push(`x</b>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 16, 0);
		wrapWith($$renderer, mf);
		$$renderer.push(`<!----></div>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
