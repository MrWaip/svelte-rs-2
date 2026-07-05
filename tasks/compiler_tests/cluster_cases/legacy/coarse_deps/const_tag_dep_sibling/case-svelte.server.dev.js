App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = $$props["obj"];
		if (obj) {
			$$renderer.push("<!--[0-->");
			const name = obj.name;
			const len = name.length;
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 8, 4);
			$$renderer.push(`${$.escape(name)}: ${$.escape(len)}</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { obj });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
