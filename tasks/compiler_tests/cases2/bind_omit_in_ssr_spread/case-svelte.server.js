import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let rest = {};
	let w = 0;
	let rect = void 0;
	let time = 0;
	let ind = false;
	let files = void 0;
	let el;
	let val = "";
	let checked = false;
	let open = false;
	$$renderer.push(`<div${$.attributes({ ...rest })}></div> <video${$.attributes({ ...rest })}></video> <input${$.attributes({
		type: "checkbox",
		...rest
	}, void 0, void 0, void 0, 4)}/> <input${$.attributes({
		type: "file",
		...rest
	}, void 0, void 0, void 0, 4)}/> <input${$.attributes({
		value: val,
		...rest
	}, void 0, void 0, void 0, 4)}/> <input${$.attributes({
		type: "checkbox",
		checked,
		...rest
	}, void 0, void 0, void 0, 4)}/> <details${$.attributes({
		open,
		...rest
	})}></details>`);
}
