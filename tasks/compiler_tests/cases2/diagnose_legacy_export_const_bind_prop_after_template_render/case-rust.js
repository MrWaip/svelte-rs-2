import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const api = () => 1;
	var $$exports = { api };
	var div = root();
	$.append($$anchor, div);
	$.bind_prop($$props, "api", api);
	return $.pop($$exports);
}
