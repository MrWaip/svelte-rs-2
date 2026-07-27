import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<b> </b><i> </i>`, 1), App[$.FILENAME], [[3, 1], [3, 18]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => $$props.items, $.index, ($$anchor, $$async0) => {
		var fragment_1 = root();
		var b_1 = $.first_child(fragment_1);
		var text = $.child(b_1, true);
		$.reset(b_1);
		var i = $.sibling(b_1);
		var text_1 = $.child(i, true);
		$.reset(i);
		$.template_effect(($0) => {
			$.set_text(text, $.get($$async0));
			$.set_text(text_1, $0);
		}, void 0, [async () => (await $.track_reactivity_loss($$props.b))()]);
		$.append($$anchor, fragment_1);
	}), "each", App, 2, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
