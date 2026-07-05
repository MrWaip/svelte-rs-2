App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"data"
]);
var root = $.add_locations($.from_html(`<object></object>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let rest = $.rest_props($$props, rest_excludes, "rest");
	var $$exports = { ...$.legacy_api() };
	var object = root();
	$.attribute_effect(object, () => ({
		data: $$props.data,
		...rest
	}));
	$.replay_events(object);
	$.append($$anchor, object);
	return $.pop($$exports);
}
