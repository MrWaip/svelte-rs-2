import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = [];
		let b = [];
		const each_array = $.ensure_array_like(a);
		if (each_array.length !== 0) {
			$$renderer.push("<!--[-->");
			for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
				let x = each_array[$$index_1];
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 2, 14);
				$$renderer.push(`${$.escape(x)}</p>`);
				$.pop_element();
			}
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push(`<!--[-->`);
			const each_array_1 = $.ensure_array_like(b);
			for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
				let x = each_array_1[$$index];
				$$renderer.push(`<span>`);
				$.push_element($$renderer, "span", 2, 45);
				$$renderer.push(`${$.escape(x)}</span>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
