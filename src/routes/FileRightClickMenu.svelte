<script lang="ts">
    import type {FileItem} from "../logics/fileitem";

    export let fileItems:FileItem[];
    export let x:number;
    export let y:number;
    export let container:HTMLDivElement;
    export let onClickFavorite:()=>void;

</script>
{#if fileItems.length > 0}
<div class="container" style="top:{y}px; left:{x}px;" bind:this={container}>
    <button on:click={onClickFavorite}>즐겨찾기</button>
</div>
{/if}

<style lang="scss">
    .container{
      position: absolute;
      border: 1px solid #cdcdcd;
      border-radius: 2px;
      background-color: white;
        padding: 10px;

    }
</style>
