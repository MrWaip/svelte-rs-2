import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[8, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let id = $.prop($$props, "id", 8, 1);
	async function fetchData(arg) {
		return arg;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => ($.deep_read_state(id()), $.untrack(() => fetchData(id()))), null, ($$anchor, v) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(v)));
		$.append($$anchor, span);
	}), "await", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
