App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1 class="svelte-1erwonp">hello</h1>`), App[$.FILENAME], [[1, 0]]);
function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var h1 = root();
	$.append($$anchor, h1);
	return $.pop($$exports);
}
if (import.meta.hot) {
	App = $.hmr(App);
	import.meta.hot.accept((module) => {
		$.cleanup_styles("svelte-1erwonp");
		App[$.HMR].update(module.default);
	});
}
export default App;
