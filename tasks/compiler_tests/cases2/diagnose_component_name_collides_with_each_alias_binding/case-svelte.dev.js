Modal_1[$.FILENAME] = "/x/Modal.svelte";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), Modal_1[$.FILENAME], [[6, 1]]);
export default function Modal_1($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, Modal_1);
	let items = $.tag_proxy($.proxy([]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => items, $.index, ($$anchor, Modal) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(Modal)));
		$.append($$anchor, p);
	}), "each", Modal_1, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
