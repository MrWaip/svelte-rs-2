App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { Foo } from "./x.js";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value } = $$props;
		if (value === Foo.X) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`a`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`b`);
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
