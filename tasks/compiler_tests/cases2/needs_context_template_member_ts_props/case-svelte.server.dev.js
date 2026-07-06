App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { Kind } from "./kinds";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const { item } = $$props;
		if (item.kind === Kind.A) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 11, 4);
			$$renderer.push(`A</span>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 13, 4);
			$$renderer.push(`B</span>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
