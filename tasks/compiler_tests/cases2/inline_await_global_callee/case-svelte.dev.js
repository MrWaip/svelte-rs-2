import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1></h1> `, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = "world";
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var h1 = $.first_child(fragment);
	h1.textContent = "Hello world!";
	var text = $.sibling(h1);
	$.template_effect(($0) => $.set_text(text, ` ${$0 ?? ""}`), void 0, [async () => (await $.track_reactivity_loss(fetch()))()]);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
