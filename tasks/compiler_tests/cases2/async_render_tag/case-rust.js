import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
const content = ($$anchor, value = $.noop) => {
	var p = root_1();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, value()));
	$.append($$anchor, p);
};
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var data;
	var $$promises = $.run([async () => data = await fetch("/api")]);
	content($$anchor, () => data);
}
