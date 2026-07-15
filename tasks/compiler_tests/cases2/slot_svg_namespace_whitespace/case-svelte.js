import * as $ from "svelte/internal/client";
import Svg from "./Svg.svelte";
var root = $.from_svg(`<path></path>`);
var root_1 = $.from_svg(`<polygon></polygon>`);
var root_2 = $.from_svg(`<!><!>`, 1);
export default function App($$anchor) {
	let paths = [];
	let polygons = [];
	Svg($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var fragment_1 = root_2();
			var node = $.first_child(fragment_1);
			$.each(node, 17, () => paths, $.index, ($$anchor, path) => {
				var path_1 = root();
				$.attribute_effect(path_1, () => ({ ...$.get(path) }));
				$.append($$anchor, path_1);
			});
			var node_1 = $.sibling(node);
			$.each(node_1, 17, () => polygons, $.index, ($$anchor, polygon) => {
				var polygon_1 = root_1();
				$.attribute_effect(polygon_1, () => ({ ...$.get(polygon) }));
				$.append($$anchor, polygon_1);
			});
			$.append($$anchor, fragment_1);
		},
		$$slots: { default: true }
	});
}
