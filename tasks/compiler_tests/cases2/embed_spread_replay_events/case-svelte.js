import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"src"
]);
var root = $.from_html(`<embed/>`);
export default function App($$anchor, $$props) {
	let rest = $.rest_props($$props, rest_excludes);
	var embed = root();
	$.attribute_effect(embed, () => ({
		src: $$props.src,
		...rest
	}));
	$.replay_events(embed);
	$.append($$anchor, embed);
}
