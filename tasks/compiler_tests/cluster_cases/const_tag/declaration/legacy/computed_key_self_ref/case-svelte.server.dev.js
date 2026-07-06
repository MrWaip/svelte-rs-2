App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let boxes = [{ k1: "a" }];
		let area = "";
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(boxes);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let box = each_array[$$index];
			const { i = 1, [`k${i}`]: sideone, [`k${area}${i + 1}`]: sidetwo } = box;
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 8, 1);
			$$renderer.push(`${$.escape(sideone)}${$.escape(sidetwo)}${$.escape(i)}</button>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
