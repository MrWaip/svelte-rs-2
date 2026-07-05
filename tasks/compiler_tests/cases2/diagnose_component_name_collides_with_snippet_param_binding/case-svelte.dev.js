Modal_1[$.FILENAME] = "/x/Modal.svelte";
import * as $ from "svelte/internal/client";
const item = $.wrap_snippet(Modal_1, function($$anchor, Modal = $.noop) {
	$.validate_snippet_args(...arguments);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, Modal()));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), Modal_1[$.FILENAME], [[6, 1]]);
export default function Modal_1($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, Modal_1);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => item($$anchor, () => "hi"), "render", Modal_1, 9, 0);
	return $.pop($$exports);
}
