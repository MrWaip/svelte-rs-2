<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';

	const { Story } = defineMeta({
		title: 'UI-KIT/Avatar',
		// 👇 Disable auto-generated documentation for this component
		tags: ['!autodocs'],
		component: Avatar,
	});
</script>

<script lang="ts">
	import { Avatar, AvatarPresets } from '../';
	import Icon from './icons/Icon.svelte';
	import IconL from './icons/IconL.svelte';
	import IconM from './icons/IconM.svelte';
	import IconS from './icons/IconS.svelte';
	import ava from './pictures/ava.jpeg';

	const { sizeXS, sizeS, sizeM, sizeL, sizeXL, sizeXXL, radius12 } = AvatarPresets;

	const initials = 'AB';
	const bgColor = 'var(--ozCtrlPrimary)';
	const bgColorGrey = 'var(--ozBGPrimary)';
	const textColor = 'var(--ozltTextPositive)';
	const bgGradient = 'var(--ozGradientAccent)';
	const image = example.com;
	const avatar = ava;
	const border = 'solid 2px rgba(91, 81, 222, 0.32)';
</script>

<Story exportName="Default" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar />
			<Avatar preset={[sizeXS]} />
			<Avatar preset={[sizeS]} />
			<Avatar preset={[sizeM]} />
			<Avatar preset={[sizeL]} />
			<Avatar preset={[sizeXL]} />
			<Avatar preset={[sizeXXL]} />
		</div>
	{/snippet}
</Story>

<Story exportName="WidthIcon" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>
			<Avatar>
				<img src={image} alt="icon" slot="icon" />
			</Avatar>

			<Avatar preset={[sizeXS]} />
			<Avatar preset={[sizeS]} />

			<Avatar preset={[sizeM]}>
				<img src={image} alt="icon" slot="icon" />
			</Avatar>

			<Avatar preset={[sizeL]} />
			<Avatar preset={[sizeXL]} />
			<Avatar preset={[sizeXXL]} />
		</div>
	{/snippet}
</Story>

<Story exportName="BgColor" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar --pprAvatarBgColor={bgColor} />
			<Avatar preset={[sizeXS]} --pprAvatarBgColor={bgColor} />
			<Avatar preset={[sizeS]} --pprAvatarBgColor={bgColor} />
			<Avatar preset={[sizeM]} --pprAvatarBgColor={bgColor} />
			<Avatar preset={[sizeL]} --pprAvatarBgColor={bgColor} />
			<Avatar preset={[sizeXL]} --pprAvatarBgColor={bgColor} />
			<Avatar preset={[sizeXXL]} --pprAvatarBgColor={bgColor} />
		</div>
	{/snippet}
</Story>

<Story exportName="BgGradient" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar --pprAvatarBgImage={bgGradient} />
			<Avatar preset={[sizeXS]} --pprAvatarBgImage={bgGradient} />
			<Avatar preset={[sizeS]} --pprAvatarBgImage={bgGradient} />
			<Avatar preset={[sizeM]} --pprAvatarBgImage={bgGradient} />
			<Avatar preset={[sizeL]} --pprAvatarBgImage={bgGradient} />
			<Avatar preset={[sizeXL]} --pprAvatarBgImage={bgGradient} />
			<Avatar preset={[sizeXXL]} --pprAvatarBgImage={bgGradient} />
		</div>
	{/snippet}
</Story>

<Story exportName="SvgSizeless" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar>
				<Icon slot="svg" />
			</Avatar>
			<Avatar preset={[sizeXS]}>
				<Icon slot="svg" />
			</Avatar>
			<Avatar preset={[sizeS]}>
				<Icon slot="svg" />
			</Avatar>
			<Avatar preset={[sizeM]}>
				<Icon slot="svg" />
			</Avatar>
			<Avatar preset={[sizeL]}>
				<Icon slot="svg" />
			</Avatar>
			<Avatar preset={[sizeXL]}>
				<Icon slot="svg" />
			</Avatar>
			<Avatar preset={[sizeXXL]}>
				<Icon slot="svg" />
			</Avatar>
		</div>
	{/snippet}
</Story>

<Story exportName="SvgWithSize" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar>
				<IconM slot="svg" />
			</Avatar>
			<Avatar preset={[sizeXS]}>
				<IconS slot="svg" />
			</Avatar>
			<Avatar preset={[sizeS]}>
				<IconS slot="svg" />
			</Avatar>
			<Avatar preset={[sizeM]}>
				<IconM slot="svg" />
			</Avatar>
			<Avatar preset={[sizeL]}>
				<IconL slot="svg" />
			</Avatar>
			<Avatar preset={[sizeXL]}>
				<IconL slot="svg" />
			</Avatar>
			<Avatar preset={[sizeXXL]}>
				<IconL slot="svg" />
			</Avatar>
		</div>
	{/snippet}
</Story>

<Story exportName="Image" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar --pprAvatarBgImage="url({image})" />
			<Avatar preset={[sizeXS]} --pprAvatarBgImage="url({image})" />
			<Avatar preset={[sizeS]} --pprAvatarBgImage="url({image})" />
			<Avatar preset={[sizeM]} --pprAvatarBgImage="url({image})" />
			<Avatar preset={[sizeL]} --pprAvatarBgImage="url({image})" />
			<Avatar preset={[sizeXL]} --pprAvatarBgImage="url({image})" />
			<Avatar preset={[sizeXXL]} --pprAvatarBgImage="url({image})" />
		</div>
	{/snippet}
</Story>

<Story exportName="Initials" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar {initials} />
			<Avatar preset={[sizeXS]} {initials} />
			<Avatar preset={[sizeS]} {initials} />
			<Avatar preset={[sizeM]} {initials} />
			<Avatar preset={[sizeL]} {initials} />
			<Avatar preset={[sizeXL]} {initials} />
			<Avatar preset={[sizeXXL]} {initials} />
		</div>
	{/snippet}
</Story>

<Story exportName="ColoredInitials" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar {initials} --pprAvatarColor={textColor} --pprAvatarBgColor={bgColor} />
			<Avatar
				preset={[sizeXS]}
				{initials}
				--pprAvatarColor={textColor}
				--pprAvatarBgColor={bgColor}
			/>
			<Avatar preset={[sizeS]} {initials} --pprAvatarColor={textColor} --pprAvatarBgColor={bgColor} />
			<Avatar preset={[sizeM]} {initials} --pprAvatarColor={textColor} --pprAvatarBgColor={bgColor} />
			<Avatar preset={[sizeL]} {initials} --pprAvatarColor={textColor} --pprAvatarBgColor={bgColor} />
			<Avatar
				preset={[sizeXL]}
				{initials}
				--pprAvatarColor={textColor}
				--pprAvatarBgColor={bgColor}
			/>
			<Avatar
				preset={[sizeXXL]}
				{initials}
				--pprAvatarColor={textColor}
				--pprAvatarBgColor={bgColor}
			/>
		</div>
	{/snippet}
</Story>

<Story exportName="Border" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar --pprAvatarBorder={border} />
			<Avatar preset={[sizeXS]} --pprAvatarBorder={border} />
			<Avatar preset={[sizeS]} --pprAvatarBorder={border} />
			<Avatar preset={[sizeM]} --pprAvatarBorder={border} />
			<Avatar preset={[sizeL]} --pprAvatarBorder={border} />
			<Avatar preset={[sizeXL]} --pprAvatarBorder={border} />
			<Avatar preset={[sizeXXL]} --pprAvatarBorder={border} />
		</div>
	{/snippet}
</Story>

<Story exportName="BorderWithColorImage" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar
				--pprAvatarBorder={border}
				--pprAvatarBgColor={bgColorGrey}
				--pprAvatarBgImage="url({avatar})"
			/>
			<Avatar
				preset={[sizeXS]}
				--pprAvatarBorder={border}
				--pprAvatarBgColor={bgColorGrey}
				--pprAvatarBgImage="url({avatar})"
			/>
			<Avatar
				preset={[sizeS]}
				--pprAvatarBorder={border}
				--pprAvatarBgColor={bgColorGrey}
				--pprAvatarBgImage="url({avatar})"
			/>
			<Avatar
				preset={[sizeM]}
				--pprAvatarBorder={border}
				--pprAvatarBgColor={bgColorGrey}
				--pprAvatarBgImage="url({avatar})"
			/>
			<Avatar
				preset={[sizeL]}
				--pprAvatarBorder={border}
				--pprAvatarBgColor={bgColorGrey}
				--pprAvatarBgImage="url({avatar})"
			/>
			<Avatar
				preset={[sizeXL]}
				--pprAvatarBorder={border}
				--pprAvatarBgColor={bgColorGrey}
				--pprAvatarBgImage="url({avatar})"
			/>
			<Avatar
				preset={[sizeXXL]}
				--pprAvatarBorder={border}
				--pprAvatarBgColor={bgColorGrey}
				--pprAvatarBgImage="url({avatar})"
			/>
		</div>
	{/snippet}
</Story>

<Story exportName="BorderWithFilledImage" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar --pprAvatarBgImage="url({image})" --pprAvatarBorder={border} />
			<Avatar preset={[sizeXS]} --pprAvatarBgImage="url({image})" --pprAvatarBorder={border} />
			<Avatar preset={[sizeS]} --pprAvatarBgImage="url({image})" --pprAvatarBorder={border} />
			<Avatar preset={[sizeM]} --pprAvatarBgImage="url({image})" --pprAvatarBorder={border} />
			<Avatar preset={[sizeL]} --pprAvatarBgImage="url({image})" --pprAvatarBorder={border} />
			<Avatar preset={[sizeXL]} --pprAvatarBgImage="url({image})" --pprAvatarBorder={border} />
			<Avatar preset={[sizeXXL]} --pprAvatarBgImage="url({image})" --pprAvatarBorder={border} />
		</div>
	{/snippet}
</Story>

<Story exportName="Radius12" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="avatar-grid">
			<b>Default</b>
			<b>XS</b>
			<b>S</b>
			<b>M</b>
			<b>L</b>
			<b>XL</b>
			<b>XXL</b>

			<Avatar preset={[radius12]} />
			<Avatar preset={[sizeXS, radius12]} />
			<Avatar preset={[sizeS, radius12]} />
			<Avatar preset={[sizeM, radius12]} />
			<Avatar preset={[sizeL, radius12]} />
			<Avatar preset={[sizeXL, radius12]} />
			<Avatar preset={[sizeXXL, radius12]} />
		</div>
	{/snippet}
</Story>

<style>
	.avatar-grid {
		display: grid;
		align-content: center;
		align-items: center;
		gap: 16px;
		grid-template-columns: repeat(7, 1fr);
		justify-items: center;
		text-align: center;
	}

	b {
		color: var(--textPrimary);
	}
</style>
