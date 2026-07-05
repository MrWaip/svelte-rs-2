import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>close</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const store = { state: { show: true } };
	const close = () => {
		store.state.show = false;
	};
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, close);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
