import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let data = $.prop($$props, "data", 24, () => [{ id: "1" }]);
	let refs = $.prop($$props, "refs", 28, () => []);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 3, data, ({ id }) => id, ($$anchor, $$item, index) => {
		let id = () => $.get($$item).id;
		id();
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.bind_this(div, ($$value, index) => $$ownership_validator.mutation(null, ["refs", index], refs(refs()[index] = $$value, true), 7, 17), (index) => refs()?.[index], () => [$.get(index)]);
		$.template_effect(() => $.set_text(text, id()));
		$.append($$anchor, div);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
