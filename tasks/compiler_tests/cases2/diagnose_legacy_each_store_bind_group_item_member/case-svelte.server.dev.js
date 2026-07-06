App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Row from "./Row.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let store = $$props["store"];
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like($.store_get($$store_subs ??= {}, "$store", store));
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let item = each_array[$$index];
				if (item.tag) {
					$$renderer.push("<!--[0-->");
					Row($$renderer, {
						get group() {
							return item.tag;
						},
						set group($$value) {
							item.tag = $$value;
							$$settled = false;
						}
					});
				} else {
					$$renderer.push("<!--[-1-->");
				}
				$$renderer.push(`<!--]-->`);
			}
			$$renderer.push(`<!--]-->`);
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { store });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
