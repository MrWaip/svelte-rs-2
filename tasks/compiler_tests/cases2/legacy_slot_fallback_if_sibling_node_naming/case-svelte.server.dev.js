App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let show = $$props["show"];
		let value = $$props["value"];
		$$renderer.push(`<li>`);
		$.push_element($$renderer, "li", 6, 0);
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "item", {}, () => {
			if (show) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<span>`);
				$.push_element($$renderer, "span", 9, 12);
				$$renderer.push(`${$.escape(value)}</span>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]--> <div>`);
			$.push_element($$renderer, "div", 11, 8);
			$$renderer.push(`tail</div>`);
			$.pop_element();
		});
		$$renderer.push(`<!--]--></li>`);
		$.pop_element();
		$.bind_props($$props, {
			show,
			value
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
