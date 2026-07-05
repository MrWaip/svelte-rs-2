App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const filters = [{ data: [1] }, { data: [2] }];
		let modeData = filters[0];
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`swap</button>`);
		$.pop_element();
		$$renderer.push(` <!--[-->`);
		const each_array = $.ensure_array_like(modeData.data);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let curtain = each_array[$$index];
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 14, 4);
			$$renderer.push(`${$.escape(curtain)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
