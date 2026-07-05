App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Icon from "./Icon.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { color } = $$props;
		let current = Icon;
		$$renderer.push(`<span class="wrap svelte-1fxeua7">`);
		$.push_element($$renderer, "span", 7, 0);
		$.css_props($$renderer, true, { "--my-color": `var(--${$.stringify(color)})` }, () => {
			if (current) {
				$$renderer.push("<!--[-->");
				current($$renderer, {});
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
