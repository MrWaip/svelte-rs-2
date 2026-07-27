import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			let number;
			var promises = $$renderer.run([async () => number = (await $.save(Promise.resolve(5)))()]);
			$$renderer.async_block([promises[0]], ($$renderer) => {
				if (number > 4) {
					$$renderer.push("<!--[0-->");
					let length;
					var promises_1 = $$renderer.run([() => promises[0], async () => length = (await $.save(number))()]);
					$$renderer.push(`<!--[-->`);
					$$renderer.async_block([promises_1[1]], ($$renderer) => {
						const each_array = $.ensure_array_like({ length });
						for (let index = 0, $$length = each_array.length; index < $$length; index++) {
							$$renderer.push(`<!---->${$.escape(index)}`);
						}
					});
					$$renderer.push(`<!--]-->`);
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
