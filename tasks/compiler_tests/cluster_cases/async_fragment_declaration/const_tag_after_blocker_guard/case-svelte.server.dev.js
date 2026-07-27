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
				let a;
				let b;
				var promises = $$renderer.run([
					() => $$promises[0],
					() => a = d() + 1,
					() => b = a + 1
				]);
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 9, 1);
				$$renderer.async([promises[2]], ($$renderer) => $$renderer.push(() => $.escape(b)));
				$$renderer.push(`</p>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
			}
		});
		$$renderer.push(`<!--]--> <button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
