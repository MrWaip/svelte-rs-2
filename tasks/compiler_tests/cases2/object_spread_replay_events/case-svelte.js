import * as $ from "svelte/internal/client";
var root = $.from_html(`<object></object>`);
export default function App($$anchor, $$props) {
	let rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"data"
	]);
	var object = root();
	$.attribute_effect(object, () => ({
		data: $$props.data,
		...rest
	}));
	$.replay_events(object);
	$.append($$anchor, object);
}
