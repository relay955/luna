<script lang="ts">
    import type { FileItem } from "../logics/fileitem";
    import {
      generateIconCacheUpdateReq,
      type IconCache,
      mergeIconCacheFromUpdateRes
    } from "../logics/iconcache";
  import LeftItemList from "./LeftItemList.svelte";
    import ListView from "./ListView.svelte";
  import TitleBar from "./TitleBar.svelte";
  import TopMenu from "./TopMenu.svelte";
  import { invoke } from "@tauri-apps/api/tauri"
    import {onMount} from "svelte";
    import type {DriveInfo} from "../logics/driveinfo";
    import FileRightClickMenu from "./FileRightClickMenu.svelte";
    import {containsNode} from "../utils/htmlnode";

  let fileRightClickMenu:HTMLDivElement;

  let searchbarMode : "filter"|"search"|"path" = "path";
  let directory = "c:/";
  let fileItems:FileItem[] = [];
  let directoryHistory:string[] = [];
  let driveList:DriveInfo[] = [];
  let selectedFileItems:FileItem[] = [];
  let favoriteFolders:FileItem[] = [];

  let rightClickFileItems:FileItem[] = [];
  let rightClickPosX = 0;
  let rightClickPosY = 0;

  let iconCache:IconCache = {
    ext: {},
    file: {},
    folderIcon: null
  };

  onMount(()=>{
    window.oncontextmenu = (e) => e.preventDefault();
    invoke("get_favorite_folders").then((res) => {
      favoriteFolders = res as FileItem[];
    });
    invoke("get_drive_list").then((res) => {
      driveList = res as DriveInfo[];
    });
  });

  $:directory, onChangeDirectory()
  const onChangeDirectory = () =>{
    let options = {
      dir:directory,
      sortBy:"name",
      sortDirection:"asc",
      groupingMode:"folder",
      search:"",
      filter:""
    }
    invoke("get_file_list", options).then(async (res) => {
      fileItems = res as FileItem[];
      let {req, reqTypeByFile} = generateIconCacheUpdateReq(iconCache, res as FileItem[]);
      let updateIconCacheRes = await invoke("get_icons", {req: req});
      iconCache = mergeIconCacheFromUpdateRes(iconCache, reqTypeByFile,updateIconCacheRes as {[index:string]:string});
    });
  }

  const onClickAllScreen = (e:MouseEvent) => {
    if (e.button === 0 && !containsNode(fileRightClickMenu, e.target as HTMLElement)) {
      rightClickFileItems = [];
    }
  }

  const onDoubleClickFileItem = (fileItem:FileItem) => {
    if(fileItem.file_type == "dir"){
      directoryHistory.push(directory);
      directory = fileItem.full_path;
    }else{
      invoke("open_file",{filePath:fileItem.full_path});
    }
  }

  const onRightClickFileItem = (fileItem:FileItem,mouseX:number, mouseY:number) => {
    rightClickFileItems = [fileItem];
    rightClickPosX = mouseX;
    rightClickPosY = mouseY;
  }

  const onClickHistoryBack = ()=>{
    if(directoryHistory.length == 0) return;
    directory = directoryHistory.pop()!;
  }

  const onClickChangeDir = (path:string) => {
    directory = path;
  }

  const onClickFavorite = () => {
    invoke("add_favorite_folder", {fullPath: rightClickFileItems[0].full_path});
    invoke("get_favorite_folders").then((res) => {
      favoriteFolders = res as FileItem[];
    });
  }

</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="container" on:mousedown={onClickAllScreen}>
  <TitleBar />
  <div style="margin-top: 30px;"></div>
  <TopMenu bind:searchbarMode={searchbarMode} bind:directory={directory}
           onClickHistoryBack={onClickHistoryBack}
   />
  <div class="inner-container">
    <LeftItemList driveList={driveList}
                  onClickChangeDir={onClickChangeDir}
                  favoriteFolders={favoriteFolders}
    />
    <div class="file-list">
      <ListView fileItems={fileItems} iconCache={iconCache} onDoubleClickFileItem={onDoubleClickFileItem}
                onRightClickFileItem={onRightClickFileItem}/>
    </div>
  </div>
    <FileRightClickMenu fileItems={rightClickFileItems}
                        x={rightClickPosX}
                        y={rightClickPosY}
                        bind:container={fileRightClickMenu}
                        onClickFavorite={onClickFavorite}
    />
</div>

<style>
  .container{
    height:100%;
    display: flex;
    flex-direction: column;
  }
  .inner-container {
    width: 100%;
    height: 0;
    flex-grow: 1;
    display: flex;
  }
  .file-list{
    flex-grow: 1;
    overflow-y: hidden;
  }
</style>
