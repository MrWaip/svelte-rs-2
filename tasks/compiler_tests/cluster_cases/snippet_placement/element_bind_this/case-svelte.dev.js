App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<form></form> `, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let thisBug = $.tag($.state(void 0), "thisBug");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var form = $.first_child(fragment);
	{
		const Bug = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text = $.text("cool");
			$.append($$anchor, text);
		});
		$.bind_this(form, ($$value) => $.set(thisBug, $$value), () => $.get(thisBug));
	}
	var text_1 = $.sibling(form);
	$.template_effect(() => $.set_text(text_1, ` ${typeof $.get(thisBug)}`));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
