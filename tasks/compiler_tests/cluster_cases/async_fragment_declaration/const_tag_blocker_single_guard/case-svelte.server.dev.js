import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let n = 1;
		var d;
		var $$promises = $$renderer.run([async () => d = await $.async_derived(() => Promise.resolve(n))]);
		$$renderer.async_block([$$promises[0]], ($$renderer) => {
			if (d()) {
				$$renderer.push("<!--[0-->");
				let v;
				var promises = $$renderer.run([() => $$promises[0], () => v = d() + 1]);
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 8, 1);
				$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(v)));
				$$renderer.push(`</p>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
			}
		});
		$$renderer.push(`<!--]--> <button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
