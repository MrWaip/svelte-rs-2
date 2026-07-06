App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 1;
		function inc() {
			count = count + 1;
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`+</button>`);
		$.pop_element();
		$$renderer.push(` `);
		if (count) {
			$$renderer.push("<!--[0-->");
			const label = count.toFixed(2);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 13, 4);
			$$renderer.push(`${$.escape(label)}</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
