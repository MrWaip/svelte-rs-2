import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let pending = null;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { pending }, ($$anchor) => {
		$.next();
		var text = $.text();
		$.template_effect(($0) => $.set_text(text, $0), void 0, [async () => (await $.track_reactivity_loss("awaited"))()]);
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
