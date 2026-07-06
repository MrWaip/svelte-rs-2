import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		async function first() {
			return false;
		}
		async function second() {
			return true;
		}
		$$renderer.child_block(async ($$renderer) => {
			if ((await $.save(first()))()) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 12, 1);
				$$renderer.push(`first</p>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.child_block(async ($$renderer) => {
					if ((await $.save(second()))()) {
						$$renderer.push("<!--[0-->");
						$$renderer.push(`<p>`);
						$.push_element($$renderer, "p", 14, 1);
						$$renderer.push(`second</p>`);
						$.pop_element();
					} else {
						$$renderer.push("<!--[-1-->");
						$$renderer.push(`<p>`);
						$.push_element($$renderer, "p", 16, 1);
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
