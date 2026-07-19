App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { rows = [] } = $$props;
		if (rows.length) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<input type="checkbox"${$.attr("checked", rows[0].check, true)}/>`);
			$.push_element($$renderer, "input", 6, 1);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { rows });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
