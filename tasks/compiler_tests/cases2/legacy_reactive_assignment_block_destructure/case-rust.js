import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let items = [{ value: 1 }, { value: 2 }];
	let source = {
		left: 3,
		right: 4
	};
	$: {
		total = 0;
		for (const item of items) {
			total += item.value;
		}
	}
	$: ({left, right} = source);
	$: if (items.length > 1) {
		conditional = total;
	} else {
		conditional = 0;
	}
	$: switch (left) {
		case 3:
			switched = right;
			break;
		default: switched = 0;
	}
	var p = root();
	p.textContent = `${total ?? ""}-${left ?? ""}-${right ?? ""}-${conditional ?? ""}-${switched ?? ""}`;
	$.append($$anchor, p);
}
