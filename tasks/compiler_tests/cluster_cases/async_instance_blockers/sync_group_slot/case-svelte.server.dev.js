import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let gate = 0;
		var one, sync1, sync2, two, sync3;
		var $$promises = $$renderer.run([
			async () => one = await $.async_derived(() => gate),
			() => {
				sync1 = gate + 1;
				sync2 = gate + 2;
			},
			async () => two = await $.async_derived(() => gate),
			() => sync3 = gate + 3
		]);
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 11, 0);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(one())));
		$$renderer.push(`</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 12, 0);
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(sync1)));
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(sync2)));
		$$renderer.push(`</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 13, 0);
		$$renderer.async([$$promises[2]], ($$renderer) => $$renderer.push(() => $.escape(two())));
		$$renderer.push(`</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 14, 0);
		$$renderer.async([$$promises[3]], ($$renderer) => $$renderer.push(() => $.escape(sync3)));
		$$renderer.push(`</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
