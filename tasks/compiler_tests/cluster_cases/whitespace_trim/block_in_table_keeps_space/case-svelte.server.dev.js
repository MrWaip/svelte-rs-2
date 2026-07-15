App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { rows } = $$props;
		$$renderer.push(`<table>`);
		$.push_element($$renderer, "table", 5, 0);
		$$renderer.push(`<tbody>`);
		$.push_element($$renderer, "tbody", 5, 7);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(rows);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let r = each_array[$$index];
			$$renderer.push(`<tr>`);
			$.push_element($$renderer, "tr", 7, 2);
			$$renderer.push(`<td>`);
			$.push_element($$renderer, "td", 7, 6);
			$$renderer.push(`${$.escape(r)}</td>`);
			$.pop_element();
			$$renderer.push(`</tr>`);
			$.pop_element();
			$$renderer.push(` <tr>`);
			$.push_element($$renderer, "tr", 8, 2);
			$$renderer.push(`<td>`);
			$.push_element($$renderer, "td", 8, 6);
			$$renderer.push(`${$.escape(r)}</td>`);
			$.pop_element();
			$$renderer.push(`</tr>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]--></tbody>`);
		$.pop_element();
		$$renderer.push(`</table>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
