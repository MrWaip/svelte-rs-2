App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { flip } from "svelte/animate";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	let items = $.tag_proxy($.proxy([{ id: 1 }, { id: 2 }]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 25, () => items, (item) => item.id, ($$anchor, item) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			$.validate_dynamic_element_tag(() => tag);
			$.validate_void_dynamic_element(() => tag);
			$.element(node_1, () => tag, false, ($$element, $$anchor) => {
				$.animation($$element, () => flip, null);
				var text = $.text();
				$.template_effect(() => $.set_text(text, $.get(item).id));
				$.append($$anchor, text);
			}, void 0, [8, 1]);
		}
		$.append($$anchor, fragment_1);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
