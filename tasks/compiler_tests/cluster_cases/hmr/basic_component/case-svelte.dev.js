App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[2, 0]]);
function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = $.tag($.state(0), "n");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(n)));
	$.delegated("click", button, function click() {
		return $.update(n);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
if (import.meta.hot) {
	App = $.hmr(App);
	import.meta.hot.accept((module) => {
		App[$.HMR].update(module.default);
	});
}
export default App;
$.delegate(["click"]);
