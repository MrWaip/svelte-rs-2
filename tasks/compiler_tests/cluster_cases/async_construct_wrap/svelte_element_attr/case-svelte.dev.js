import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function g() {
		return 2;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => $$props.tag);
		$.element(node, () => $$props.tag, false, ($$element, $$anchor) => {
			$.attribute_effect($$element, ($0) => ({ title: $0 }), void 0, [async () => (await $.track_reactivity_loss(g()))()]);
		}, void 0, [5, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
