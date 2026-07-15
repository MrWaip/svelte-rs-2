App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = $$props["a"];
		let b = $$props["b"];
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			if (a) {
				$$renderer.push("<!--[0-->");
				if (b) {
					$$renderer.push("<!--[0-->");
					$$renderer.title(($$renderer) => {
						$$renderer.push(`<title>deep</title>`);
					});
				} else {
					$$renderer.push("<!--[-1-->");
				}
				$$renderer.push(`<!--]-->`);
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		});
		$.bind_props($$props, {
			a,
			b
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
