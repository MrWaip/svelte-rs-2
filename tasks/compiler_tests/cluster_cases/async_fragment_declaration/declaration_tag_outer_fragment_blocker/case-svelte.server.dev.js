import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let n = 1;
		if (n) {
			$$renderer.push("<!--[0-->");
			let outer;
			var promises = $$renderer.run([async () => outer = await $.async_derived(async () => (await $.save(Promise.resolve(n)))())]);
			{
				let inner;
				var promises_1 = $$renderer.run([() => promises[0], () => inner = $.derived(() => `v${outer()}`)]);
				$$renderer.push(`<div>`);
				$.push_element($$renderer, "div", 7, 1);
				$$renderer.push(`<span>`);
				$.push_element($$renderer, "span", 9, 2);
				$$renderer.async([promises_1[1]], ($$renderer) => $$renderer.push(() => $.escape(inner())));
				$$renderer.push(`</span>`);
				$.pop_element();
				$$renderer.push(`</div>`);
				$.pop_element();
			}
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> <button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
