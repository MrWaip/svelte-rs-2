App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { cond = true, show = true } = $$props;
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			if (show) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<meta name="a" content="b"/>`);
				$.push_element($$renderer, "meta", 10, 8);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		});
		if (cond) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<button>`);
			$.push_element($$renderer, "button", 6, 4);
			$$renderer.push(`x</button>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
