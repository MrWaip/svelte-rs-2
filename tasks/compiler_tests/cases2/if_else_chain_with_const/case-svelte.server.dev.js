App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		let name = "world";
		if (count > 100) {
			$$renderer.push("<!--[0-->");
			const label = name + "!";
			$$renderer.push(`<h1>`);
			$.push_element($$renderer, "h1", 8, 4);
			$$renderer.push(`world!</h1>`);
			$.pop_element();
		} else if (count > 50) {
			$$renderer.push("<!--[1-->");
			$$renderer.push(`<h2>`);
			$.push_element($$renderer, "h2", 10, 4);
			$$renderer.push(`Medium: 0</h2>`);
			$.pop_element();
		} else if (count > 10) {
			$$renderer.push("<!--[2-->");
			const small = count * 2;
			$$renderer.push(`<h3>`);
			$.push_element($$renderer, "h3", 13, 4);
			$$renderer.push(`Small doubled: 0</h3>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 15, 4);
			$$renderer.push(`Tiny: 0</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
