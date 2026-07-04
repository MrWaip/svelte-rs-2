import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[8, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $store = () => ($.validate_store(store(), "store"), $.store_get(store(), "$store", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = $.prop($$props, "store", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, $store, (item) => item.id, ($$anchor, item, $$index) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			var consequent = ($$anchor) => {
				var input = root();
				$.remove_input_defaults(input);
				$.bind_checked(input, function get() {
					return $.get(item).enabled;
				}, function set($$value) {
					$.get(item).enabled = $$value, $.invalidate_inner_signals(() => $store()), $.invalidate_store($$stores, "$store");
				});
				$.append($$anchor, input);
			};
			$.add_svelte_meta(() => $.if(node_1, ($$render) => {
				if ($.get(item), $.untrack(() => $.get(item).enabled)) $$render(consequent);
			}), "if", App, 7, 1);
		}
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
