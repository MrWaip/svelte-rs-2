App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[2, 0]]);
function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
if (import.meta.hot) {
	App = $.hmr(App);
	import.meta.hot.accept((module) => {
		App[$.HMR].update(module.default);
	});
}
export default App;
if (customElements.get("my-el") == null) customElements.define("my-el", $.create_custom_element(App, {}, [], [], { mode: "open" }));
