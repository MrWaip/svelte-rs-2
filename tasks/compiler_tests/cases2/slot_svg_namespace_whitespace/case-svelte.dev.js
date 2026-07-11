App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Svg from "./Svg.svelte";
var root = $.add_locations($.from_svg(`<path></path>`), App[$.FILENAME], [[10, 2]]);
var root_1 = $.add_locations($.from_svg(`<polygon></polygon>`), App[$.FILENAME], [[13, 2]]);
var root_2 = $.add_locations($.from_svg(`<!><!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let paths = [];
	let polygons = [];
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Svg($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var fragment_1 = root_2();
			var node = $.first_child(fragment_1);
			$.add_svelte_meta(() => $.each(node, 17, () => paths, $.index, ($$anchor, path) => {
				var path_1 = root();
				$.attribute_effect(path_1, () => ({ ...$.get(path) }));
				$.append($$anchor, path_1);
			}), "each", App, 9, 1);
			var node_1 = $.sibling(node);
			$.add_svelte_meta(() => $.each(node_1, 17, () => polygons, $.index, ($$anchor, polygon) => {
				var polygon_1 = root_1();
				$.attribute_effect(polygon_1, () => ({ ...$.get(polygon) }));
				$.append($$anchor, polygon_1);
			}), "each", App, 12, 1);
			$.append($$anchor, fragment_1);
		}),
		$$slots: { default: true }
	}), "component", App, 8, 0, { componentTag: "Svg" });
	return $.pop($$exports);
}
