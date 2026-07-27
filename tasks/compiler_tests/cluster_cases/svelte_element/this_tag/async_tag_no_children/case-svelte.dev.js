import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function getTag() {
		return "div";
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [async () => (await $.track_reactivity_loss(getTag()))()], (node, $$tag) => {
		$.validate_dynamic_element_tag(() => $.get($$tag));
		$.element(node, () => $.get($$tag), false, void 0, void 0, [7, 0]);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
