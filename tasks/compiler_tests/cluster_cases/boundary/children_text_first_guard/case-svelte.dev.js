App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>f</p>`), App[$.FILENAME], [[8, 20]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var p = root();
			$.append($$anchor, p);
		});
		$.boundary(node, { failed }, ($$anchor) => {
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, `boundary ${$$props.x ?? ""} text`));
			$.append($$anchor, text);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
