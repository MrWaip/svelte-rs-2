import * as $ from "svelte/internal/client";
var root = $.from_html(`<embed/>`);
export default function App($$anchor, $$props) {
	let rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"src"
	]);
	var embed = root();
	$.attribute_effect(embed, () => ({
		src: $$props.src,
		...rest
	}));
	$.replay_events(embed);
	$.append($$anchor, embed);
}
