import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		async function h() {
			return 1;
		}
		async function f() {
			return 2;
		}
		$$renderer.child(async ($$renderer) => {
			const [$$0, $$1] = (await $.save(Promise.all([(async () => (await $.save(h()))())(), (async () => (await $.save(f()))())()])))();
			$$renderer.push(`<div${$.attr("title", `z${$.stringify($$0)}`)}${$.attr_class("", void 0, { "a": $$1 })}>`);
			$.push_element($$renderer, "div", 5, 0);
			$$renderer.push(`</div>`);
			$.pop_element();
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
