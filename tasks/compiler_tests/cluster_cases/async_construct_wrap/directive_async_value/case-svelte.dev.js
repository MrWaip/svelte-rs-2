import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>one</div> <div>two</div> <div>red</div> <div>blue</div>`, 1), App[$.FILENAME], [
	[2, 1],
	[3, 1],
	[5, 1],
	[6, 1]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text = $.text("loading");
			$.append($$anchor, text);
		});
		$.boundary(node, { pending }, ($$anchor) => {
			var fragment_1 = root();
			var div = $.first_child(fragment_1);
			$.attribute_effect(div, ($0) => ({
				...{},
				[$.CLASS]: $0
			}), void 0, [async () => ({ one: (await $.track_reactivity_loss(true))() })]);
			var div_1 = $.sibling(div, 2);
			let classes;
			var div_2 = $.sibling(div_1, 2);
			$.attribute_effect(div_2, ($0) => ({
				...{},
				[$.STYLE]: $0
			}), void 0, [async () => ({ color: (await $.track_reactivity_loss("red"))() })]);
			var div_3 = $.sibling(div_2, 2);
			let styles;
			$.template_effect(($0, $1) => {
				classes = $.set_class(div_1, 1, "", null, classes, $0);
				styles = $.set_style(div_3, "", styles, $1);
			}, void 0, [async () => ({ two: (await $.track_reactivity_loss(true))() }), async () => ({ color: (await $.track_reactivity_loss("blue"))() })]);
			$.append($$anchor, fragment_1);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
