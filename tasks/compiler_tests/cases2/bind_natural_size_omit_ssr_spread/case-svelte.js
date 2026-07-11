import * as $ from "svelte/internal/client";
var root = $.from_html(`<img/>`);
export default function App($$anchor) {
	let rest = {};
	let nw = $.state(0);
	let nh = $.state(0);
	var img = root();
	$.attribute_effect(img, () => ({
		alt: "",
		...rest
	}));
	$.replay_events(img);
	$.bind_property("naturalWidth", "load", img, ($$value) => $.set(nw, $$value));
	$.bind_property("naturalHeight", "load", img, ($$value) => $.set(nh, $$value));
	$.append($$anchor, img);
}
