App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<img/>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let items = $.prop($$props, "items", 7);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, items, $.index, ($$anchor, item, i) => {
		var img = root();
		$.set_attribute(img, "alt", `slider${i}`);
		$.validate_binding("bind:this={items[i].img}", [], () => items()[i], () => "img", 6, 21);
		$.bind_this(img, ($$value, i) => $$ownership_validator.mutation("items", [
			"items",
			i,
			"img"
		], items()[i].img = $$value, 6, 32), (i) => items()?.[i]?.img, () => [i]);
		$.template_effect(() => $.set_attribute(img, "src", $.get(item).src));
		$.append($$anchor, img);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
