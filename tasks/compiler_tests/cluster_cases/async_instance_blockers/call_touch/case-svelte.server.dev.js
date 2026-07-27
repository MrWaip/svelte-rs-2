import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let gate = true;
		var loaded;
		var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate)]);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 6, 0);
		$$renderer.push(`toggle</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.async_block([$$promises[0]], ($$renderer) => {
			if (gate) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`yes`);
			} else {
				$$renderer.push("<!--[-1-->");
			}
		});
		$$renderer.push(`<!--]--> <p>`);
		$.push_element($$renderer, "p", 8, 0);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded())));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
