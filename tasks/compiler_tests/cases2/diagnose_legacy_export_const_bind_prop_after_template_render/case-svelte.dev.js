import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const api = () => 1;
	var $$exports = {
		...$.legacy_api(),
		get api() {
			return api;
		}
	};
	var div = root();
	$.append($$anchor, div);
	$.bind_prop($$props, "api", api);
	return $.pop($$exports);
}
