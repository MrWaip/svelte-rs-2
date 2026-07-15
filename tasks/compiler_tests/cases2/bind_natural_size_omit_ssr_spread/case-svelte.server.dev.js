App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rest = {};
		let nw = 0;
		let nh = 0;
		$$renderer.push(`<img${$.attributes({
			alt: "",
			...rest
		})} onload="this.__e=event" onerror="this.__e=event"/>`);
		$.push_element($$renderer, "img", 7, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
