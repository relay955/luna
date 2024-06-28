<script lang="ts">
  import type {DriveInfo} from "../logics/driveinfo";
  import ProgressBar from "./common/ProgressBar.svelte";
  import {beutifyFileSize} from "../utils/filesizeBeutifier";
  import type {FileItem} from "../logics/fileitem";
  export let driveList:DriveInfo[] = [];
  export let favoriteFolders:FileItem[] = [];
  export let onClickChangeDir:(path:string)=>void;

</script>

<div class="left-item-list">
    <div class="section-header"> Drives </div>
    {#each driveList as drive}
        <button class="drive-item" on:click={()=>onClickChangeDir(drive.full_path)}>
            <div class="drive-name">{drive.name} ({drive.full_path})</div>
            <div class="drive-space">{`${beutifyFileSize(drive.total_space - drive.free_space)}/${beutifyFileSize(drive.total_space)} (${beutifyFileSize(drive.free_space)} left)`}</div>
            <ProgressBar value={(drive.total_space - drive.free_space) / drive.total_space * 100} height={5} />
        </button>
    {/each}
    <div class="section-header"> Favorites </div>
    {#each favoriteFolders as folder}
        <button class="favorite-item" on:click={()=>onClickChangeDir(folder.full_path)}>
            <div class="drive-name
            ">{folder.name}</div>
        </button>
    {/each}
</div>

<style lang="scss">
  .favorite-item{
    width: 100%;
    text-align: left;
    font-size: 12px;
    cursor: pointer;
    border:none;
    border-radius: 3px;
    margin-top: 0;
    margin-bottom: 3px;
    transition: all 100ms;
    background: transparent;
    &:hover{
      background-color: #f0f0f0;
    }
  }
  .drive-item{
    width: 100%;
    text-align: left;
    padding: 2px 4px;
    cursor: pointer;
    border: 1px solid #e8e8e8;
    border-radius: 3px;
    margin-top: 0;
    margin-bottom: 3px;
    background-color: white;
    transition: all 100ms;
    .drive-name{
        font-size: 12px;
        color: #666;
    }
    .drive-space{
        font-size: 10px;
        color: #999;

    }

    &:hover{
      background-color: #f0f0f0;
    }
  }
  .section-header{
    font-size: 12px;
    color: #666;
    font-weight: bold;
  }
  .left-item-list {
    padding: 5px;
    width: 200px;
    height: 100%;
    background: rgb(252, 252, 252);
    z-index: 0;
    border-right: 1px solid #e8e8e8;
    box-sizing: border-box;
  }
</style>
