<script>
	let fruit = $state('apple');
	let veggie = $state('carrot');
</script>

<select>
	<optgroup label="Fruits">
		<span class="fh">{fruit}</span>
		<option value="a"><span>{fruit}</span></option>
	</optgroup>
	<optgroup label="Vegs">
		<em class="vh">{veggie}</em>
		<option value="c"><em>{veggie}</em></option>
	</optgroup>
</select>

<button onclick={() => { fruit = 'orange'; veggie = 'broccoli'; }}>x</button>
