App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let src = "";
		let cond = false;
		function on_load() {}
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			if (cond) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<script async=""${$.attr("src", src)} onload="this.__e=event"><\/script>`);
				$$renderer.push(`<!---->`);
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
