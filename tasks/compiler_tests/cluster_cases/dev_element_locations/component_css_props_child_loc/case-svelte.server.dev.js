App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { Icon } = $$props;
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 5, 0);
		$.css_props($$renderer, true, { "--color": "red" }, () => {
			if (Icon) {
				$$renderer.push("<!--[-->");
				Icon($$renderer, {});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		}, true);
		$$renderer.push(`</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
