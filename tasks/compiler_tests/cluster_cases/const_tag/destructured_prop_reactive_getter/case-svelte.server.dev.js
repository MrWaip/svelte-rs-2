App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { data } = $$props;
		if (data) {
			$$renderer.push("<!--[0-->");
			const simpleReactive = data.foo;
			const { destr } = { destr: 1 };
			const simpleStatic = 5;
			Child($$renderer, {
				a: simpleReactive,
				b: destr,
				c: simpleStatic
			});
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
