App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import A from "./A.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const B = $.derived(() => A);
		if (B()) {
			$$renderer.push("<!--[-->");
			B()($$renderer, {
				children: $.prevent_snippet_stringification(($$renderer) => {
					$$renderer.push(`<!---->test`);
				}),
				$$slots: { default: true }
			});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
