import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let props = $.tag_proxy($.proxy({ id: "x" }), "props");
	async function getTag() {
		return "div";
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [async () => (await $.track_reactivity_loss(getTag()))()], (node, $$tag) => {
		$.validate_dynamic_element_tag(() => $.get($$tag));
		$.validate_void_dynamic_element(() => $.get($$tag));
		$.element(node, () => $.get($$tag), false, ($$element, $$anchor) => {
			$.attribute_effect($$element, () => ({ ...props }));
			var text = $.text("x");
			$.append($$anchor, text);
		}, void 0, [6, 0]);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
