App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { flip } from "svelte/animate";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { items = [], $$slots, $$events, ...rest } = $$props;
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let idx = 0, $$length = each_array.length; idx < $$length; idx++) {
			let item = each_array[idx];
			$$renderer.push(`<p${$.attributes({
				...rest,
				"data-index": `item-${$.stringify(idx)}`
			})}>`);
			$.push_element($$renderer, "p", 7, 1);
			$$renderer.push(`${$.escape(item.name)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
