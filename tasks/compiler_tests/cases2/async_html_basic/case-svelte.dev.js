import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function loadContent() {
		return "<b>hello</b>";
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [async () => (await $.track_reactivity_loss(loadContent()))()], (node, $$html) => {
		$.html(node, () => $.get($$html));
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
