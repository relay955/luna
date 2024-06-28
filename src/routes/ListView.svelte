<script lang="ts">
    import { slide } from "svelte/transition";
    import type { FileItem } from "../logics/fileitem";
    import { beutifyFileSize } from "../utils/filesizeBeutifier";
    import {getIconBinary, type IconCache} from "../logics/iconcache";

    export let fileItems = [] as FileItem[];

    export let iconCache:IconCache;

    export let onDoubleClickFileItem: (fileItem:FileItem)=>void;
    export let onRightClickFileItem: (fileItem:FileItem,mouseX:number, mouseY:number)=>void;

    const onMouseDownFileItem = (e:MouseEvent, fileItem:FileItem)=>{
        if(e.button === 2){
            onRightClickFileItem(fileItem,e.clientX,e.clientY);
        }
    }

</script>

<div class="root">
    <table>
        <thead>
        <tr>
            <th>Name</th>
            <th>Edit Date</th>
            <th>Size</th>
            <th>Type</th>
        </tr>
        </thead>
        <tbody>
            {#each fileItems as fileItem}
            <tr on:dblclick={()=>onDoubleClickFileItem(fileItem)} on:mousedown={(e)=>onMouseDownFileItem(e,fileItem)}>
            <td class="td-name">
                <div style="display: flex; align-items: center;">
                <img src={`data:image/png;base64,${getIconBinary(iconCache,fileItem)}`} alt="file_icon" width="20px" height="20px" style="margin-right:5px;"/>
                {fileItem.name}
                </div>
            </td>
            <td class="td-date">{fileItem.edit_date}</td>
            <td class="td-size">{beutifyFileSize(fileItem.size)}</td>
            <td class="td-type">{fileItem.file_type}</td>
            </tr>
            {/each}
        </tbody>
    </table>
</div>

<style lang="scss">
    .root{
        height: 100%;
        overflow-y: auto;
    }
    table {
        width: 100%;
        border-collapse: collapse;
    }

    th, td {
        padding: 2px 4px;
        text-align: left;
    }

    tr > th {
        border-bottom: 1px solid #ddd;
        position: sticky;
        top: 0;
    }

    th {
        background-color: #f5f5f5;
        font-size: 14px;
    }

    tr{
        transition: 300ms all;
        &:nth-child(even){
            background-color: #f5f5f5;
        }
        &:hover{
            background-color: #e5e5e5;
        }
    }


    .td-name {
        width: 400px;
        font-size: 12px;
    }
    .td-date {
        width: 200px;
        font-size: 12px;
    }
    .td-size{
        width: 100px;
        font-size: 12px;
    }


</style>
