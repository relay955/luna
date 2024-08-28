<script lang="ts">
    import type {FileItem} from "../logics/fileitem";

    export let fileItems:FileItem[];
    export let x:number;
    export let y:number;
    export let container:HTMLDivElement;
    export let onClickFavorite:()=>void;
    export let onClickEncrypt:()=>void;

</script>
{#if fileItems.length > 0}
<div class="container" style="top:{y}px; left:{x}px;" bind:this={container}>
  <div class="section">File</div>
  <button on:click={onClickFavorite}>즐겨찾기</button>
  <button on:click={onClickEncrypt}>보호</button>
</div>
{/if}

<style lang="scss">
  .container {
    position: absolute;
    border: 1px solid #cdcdcd;
    border-radius: 4px;
    background-color: white;
    min-width: 200px;
    min-height: 200px;
    .section{
      padding: 5px;
      font-size: 10px;
      font-weight: bold;
    }

    button {
      width: 100%;
      text-align: left;
      padding: 5px 10px;
      border-top: 1px solid #cdcdcd;
      border-bottom: none;
      border-left: none;
      border-right: none;
      background-color: #f5f5f5;
      cursor: pointer;
      transition: all 200ms;

      &:hover {
        background-color: #f0f0f0;
      }
    }
  }

</style>
