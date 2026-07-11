App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let condition = $$props["condition"];
		let name = $$props["name"];
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			if (condition) {
				$$renderer.push("<!--[0-->");
				$$renderer.title(($$renderer) => {
					$$renderer.push(`<title>Hi ${$.escape(name)}</title>`);
				});
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		});
		$.bind_props($$props, {
			condition,
			name
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
