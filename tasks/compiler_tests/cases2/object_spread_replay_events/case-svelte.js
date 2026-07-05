import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"data"
]);
var root = $.from_html(`<object></object>`);
export default function App($$anchor, $$props) {
	let rest = $.rest_props($$props, rest_excludes);
	var object = root();
	$.attribute_effect(object, () => ({
		data: $$props.data,
		...rest
	}));
	$.replay_events(object);
	$.append($$anchor, object);
}
