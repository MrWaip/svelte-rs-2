import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		async function check(v) {
			return v > 0;
		}
		$$renderer.child_block(async ($$renderer) => {
			if ((await $.save(check(count)))()) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 10, 1);
				$$renderer.push(`yes</p>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
			}
		});
		$$renderer.push(`<!--]--> <button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
