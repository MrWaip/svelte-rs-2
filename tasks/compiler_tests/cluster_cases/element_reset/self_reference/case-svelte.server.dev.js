App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let file = $$props["file"];
		$$renderer.push(`<article${$.attr_class(`file ${$.stringify(file.type)}`)}>`);
		$.push_element($$renderer, "article", 5, 0);
		$$renderer.push(`<span class="name">`);
		$.push_element($$renderer, "span", 6, 1);
		$$renderer.push(`${$.escape(file.name)}</span>`);
		$.pop_element();
		$$renderer.push(` `);
		if (file.type === "folder") {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<ul>`);
			$.push_element($$renderer, "ul", 9, 2);
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like(file.children);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let child = each_array[$$index];
				$$renderer.push(`<li>`);
				$.push_element($$renderer, "li", 11, 4);
				App($$renderer, { file: child });
				$$renderer.push(`<!----></li>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]--></ul>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--></article>`);
		$.pop_element();
		$.bind_props($$props, { file });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
