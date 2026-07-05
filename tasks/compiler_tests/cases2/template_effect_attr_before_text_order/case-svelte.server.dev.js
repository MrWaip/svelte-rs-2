App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { f, g } from "./x";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { url = "", label = "" } = $$props;
		$$renderer.push(`<a${$.attr("href", f(url))}>`);
		$.push_element($$renderer, "a", 6, 0);
		$$renderer.push(`${$.escape(g(label))}</a>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
