App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let classes = [];
		function mapClasses(base, ...rest) {
			return { [base]: true };
		}
		$$renderer.push(`<div${$.attributes({ ...mapClasses("base", ...classes) }, void 0, { active: true })}>`);
		$.push_element($$renderer, "div", 8, 0);
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
