import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var a;
		var $$promises = $$renderer.run([async () => a = await Promise.resolve(1)]);
		$$renderer.push(`<!--[-->`);
		{
			let b;
			var promises = $$renderer.run([async () => b = (await $.save(Promise.resolve(2)))()]);
			$$renderer.async_block([promises[0], $$promises[0]], ($$renderer) => {
				if (b + a > 0) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<p>`);
					$.push_element($$renderer, "p", 6, 16);
					$$renderer.push(`yes</p>`);
					$.pop_element();
				} else {
					$$renderer.push("<!--[-1-->");
				}
			});
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
