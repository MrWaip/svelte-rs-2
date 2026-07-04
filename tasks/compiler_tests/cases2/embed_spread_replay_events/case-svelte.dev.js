App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"src"
]);
var root = $.add_locations($.from_html(`<embed/>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let rest = $.rest_props($$props, rest_excludes, "rest");
	var $$exports = { ...$.legacy_api() };
	var embed = root();
	$.attribute_effect(embed, () => ({
		src: $$props.src,
		...rest
	}));
	$.replay_events(embed);
	$.append($$anchor, embed);
	return $.pop($$exports);
}
