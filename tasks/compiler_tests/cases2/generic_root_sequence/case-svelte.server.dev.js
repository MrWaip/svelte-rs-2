App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<!---->some text <div>`);
		$.push_element($$renderer, "div", 2, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` ${$.escape(some_variable)} <input/>`);
		$.push_element($$renderer, "input", 4, 0);
		$.pop_element();
		$$renderer.push(` text + ${$.escape(name)} <div>`);
		$.push_element($$renderer, "div", 6, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
		$$renderer.push(` `);
		if (true) {
			$$renderer.push("<!--[0-->");
		} else if (false) {
			$$renderer.push("<!--[1-->");
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
