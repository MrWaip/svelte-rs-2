import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var a, b;
		var $$promises = $$renderer.run([async () => a = await first_fetch(), async () => b = await second_fetch()]);
		$$renderer.async_block([$$promises[0]], ($$renderer) => {
			if (a) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 7, 1);
				$$renderer.push(`a</p>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.async_block([$$promises[1]], ($$renderer) => {
					if (b) {
						$$renderer.push("<!--[0-->");
						$$renderer.push(`<p>`);
						$.push_element($$renderer, "p", 9, 1);
						$$renderer.push(`b</p>`);
						$.pop_element();
					} else {
						$$renderer.push("<!--[-1-->");
						$$renderer.push(`<p>`);
						$.push_element($$renderer, "p", 11, 1);
						$$renderer.push(`fallback</p>`);
						$.pop_element();
					}
				});
				$$renderer.push(`<!--]-->`);
			}
		});
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
