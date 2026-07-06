import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let left, right;
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
	$$renderer.push(`<p>${$.escape(total)}-${$.escape(left)}-${$.escape(right)}-${$.escape(conditional)}-${$.escape(switched)}</p>`);
}
