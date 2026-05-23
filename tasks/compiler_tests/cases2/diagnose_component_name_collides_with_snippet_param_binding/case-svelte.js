import * as $ from "svelte/internal/client";
const item = ($$anchor, Modal = $.noop) => {
	var p = root_1();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, Modal()));
	$.append($$anchor, p);
};
var root_1 = $.from_html(`<p> </p>`);
export default function Modal_1($$anchor, $$props) {
	item($$anchor, () => "hi");
}
