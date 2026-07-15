import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<textarea></textarea> <div contenteditable="true"></div>`, 1), App[$.FILENAME], [[6, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $name = () => ($.validate_store(name, "name"), $.store_get(name, "$name", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const name = writable("world");
	var $$exports = {
		...$.legacy_api(),
		get name() {
			return name;
		}
	};
	$.init();
	var fragment = root();
	var textarea = $.first_child(fragment);
	$.remove_textarea_child(textarea);
	var div = $.sibling(textarea, 2);
	$.bind_value(textarea, function get() {
		return $name();
	}, function set($$value) {
		$.store_set(name, $$value);
	});
	$.bind_content_editable("innerHTML", div, function get() {
		return $name();
	}, function set($$value) {
		$.store_set(name, $$value);
	});
	$.append($$anchor, fragment);
	$.bind_prop($$props, "name", name);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
