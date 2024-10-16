<script lang="ts">
	import type { Structure } from '$types/Structure';
	interface CollapseItem {
		collapsed: boolean;
		substructs?: CollapseItem[];
	}

	export let structures: Structure[] = [];
	let collapse_arr: CollapseItem[] = [];

	$: collapse_arr = createCollapseArr(structures);
	// $: console.log(collapse_arr);

	// Function to toggle the visibility of a section
	function toggleSection(...args: number[]) {
		let i = args[0];
		let j = args.length > 1 ? args[1] : undefined;

		if (j === undefined) {
			collapse_arr[i].collapsed = !collapse_arr[i].collapsed;
		} else {
			if (collapse_arr[i].substructs) {
				collapse_arr[i].substructs[j].collapsed = !collapse_arr[i].substructs[j].collapsed;
			}
		}
	}

	// Determine the visibility of a section
	function determineVisibility(collapse_arr: CollapseItem[], ...args: number[]): boolean {
		let i = args[0];
		let j = args.length > 1 ? args[1] : undefined;

		if (j === undefined) {
			return collapse_arr[i].collapsed;
		} else {
			if (collapse_arr[i].substructs) {
				return collapse_arr[i].substructs[j].collapsed;
			} else {
				return false;
			}
		}
	}

	// Function for creating recursirve structure of boolean to match the strucutre
	function createCollapseArr(structure: Structure[]): CollapseItem[] {
		return structure.map((item) => ({
			collapsed: false,
			substructs: item.substructs ? createCollapseArr(item.substructs) : undefined
		}));
	}
</script>

<table class="table table-bordered">
	<thead>
		<tr class="table-section-header">
			<th>Attribute Name</th>
			<th>Attribute Value</th>
			<th>Description</th>
		</tr>
	</thead>

	{#each structures as structure, i}
		<thead>
			<tr class="table-section-header" on:click={() => toggleSection(i)}>
				<th colspan="3">{structure.name}</th>
			</tr>
		</thead>
		<tbody
			id="section2"
			class="table-section-content"
			class:show={determineVisibility(collapse_arr, i)}
		>
			{#each structure.structure as structure_item}
				<tr style="max-width: 600px">
					<td>{structure_item.name}</td>
					<td style="max-width: inherit;  word-wrap: break-word;">{structure_item.value}</td>
					{#if structure_item.description}
						<td style="max-width: inherit;">{structure_item.description}</td>
					{:else}
						<td></td>
					{/if}
				</tr>
			{/each}
			{#if structure.substructs}
				{@const subsection = structure.substructs}
				<tr>
					<td colspan="3">
						<table class="table table-bordered" style="width: 98%; margin-left: 10px">
							{#each subsection as section, j}
								<thead>
									<tr class="table-section-header" on:click={() => toggleSection(i, j)}>
										<th colspan="3">{section.name}</th>
									</tr>
								</thead>
								<tbody
									id="section2"
									class="table-section-content"
									class:show={determineVisibility(collapse_arr, i, j)}
								>
									{#each section.structure as structure_item}
										<tr>
											<td id="name">{structure_item.name}</td>
											<td id="value">{structure_item.value}</td>
											{#if structure_item.description}
												<td id="desc">{structure_item.description}</td>
											{:else}
												<td id="desc"></td>
											{/if}
										</tr>
									{/each}
								</tbody>
							{/each}
						</table>
					</td>
				</tr>
			{/if}
		</tbody>
	{/each}
</table>

<style>
	.table-section-header {
		background-color: #808080;
		color: white;
		cursor: pointer;
	}
	.table-section-content {
		display: none;
		background-color: #d3d3d3;
	}
	.table-section-content > tr > td {
		padding: 0.2rem;
	}
	.table-section-content.show {
		display: table-row-group;
	}
	th {
		width: 35%;
	}
	#name {
		width: 25%;
	}
	#value {
		width: 25%;
	}
	#desc {
		width: 50%;
	}
</style>
