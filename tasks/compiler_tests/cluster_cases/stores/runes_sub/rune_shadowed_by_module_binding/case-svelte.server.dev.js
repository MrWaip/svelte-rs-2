App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
const data = 0;
const derived = $.derived(() => data + 1);
const state = {
	data,
	derived: derived()
};
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		console.log("$inspect(", state, ")");
		$$renderer.push(`<!---->${$.escape(state.data)}`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
