import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"src"
]);
var root = $.from_html(`<img/>`);
export default function App($$anchor, $$props) {
	let rest = $.rest_props($$props, rest_excludes);
	var img = root();
	$.attribute_effect(img, () => ({
		src: $$props.src,
		...rest
	}));
	$.replay_events(img);
	$.append($$anchor, img);
}
