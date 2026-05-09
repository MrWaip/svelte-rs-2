import * as $ from "svelte/internal/client";
var root = $.from_html(`<img/>`);
export default function App($$anchor, $$props) {
	let rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"src"
	]);
	var img = root();
	$.attribute_effect(img, () => ({
		src: $$props.src,
		...rest
	}));
	$.replay_events(img);
	$.append($$anchor, img);
}
