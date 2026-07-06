App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { src, $$slots, $$events, ...rest } = $$props;
		$$renderer.push(`<img${$.attributes({
			src,
			...rest
		})} onload="this.__e=event" onerror="this.__e=event"/>`);
		$.push_element($$renderer, "img", 5, 0);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
