App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { $$slots, $$events, ...props } = $$props;
		$.prevent_snippet_stringification(body);
		function body($$renderer) {
			$.validate_snippet_args($$renderer);
			if (props.Inner) {
				$$renderer.push("<!--[0-->");
				if (props.Inner) {
					$$renderer.push("<!--[-->");
					props.Inner($$renderer, {});
					$$renderer.push("<!--]-->");
				} else {
					$$renderer.push("<!--[!-->");
					$$renderer.push("<!--]-->");
				}
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		}
		Child($$renderer, { icon: props.show ? body : undefined });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
