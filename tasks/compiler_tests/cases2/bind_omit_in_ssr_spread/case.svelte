<script>
	let rest = {};
	let w = $state(0);
	let rect = $state();
	let time = $state(0);
	let ind = $state(false);
	let files = $state();
	let el;
	let val = $state('');
	let checked = $state(false);
	let open = $state(false);
</script>

<div bind:clientWidth={w} bind:contentRect={rect} bind:this={el} {...rest}></div>
<video bind:currentTime={time} {...rest}></video>
<input type="checkbox" bind:indeterminate={ind} {...rest} />
<input type="file" bind:files {...rest} />

<input bind:value={val} {...rest} />
<input type="checkbox" bind:checked {...rest} />
<details bind:open {...rest}></details>
