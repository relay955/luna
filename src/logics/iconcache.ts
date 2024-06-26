import type { FileItem } from './fileitem';

export interface IconCache{
    ext:{[index:string]:string};
    file:{[index:string]:string};
    folderIcon:string | null;
}

export type TypeByFile = {[index:string]:"dir"|"ext"|"file"};

/**
 * 현재 캐시에 담겨져있는 아이콘 정보와 파일 정보를 비교하여 업데이트가 필요한 아이콘 정보를 요청하는 요청을 생성합니다.
 */
export const generateIconCacheUpdateReq = (cache:IconCache, files:FileItem[]) => {
    let req:String[] = [];
    let reqTypeByFile:TypeByFile = {};
    let isFolderIconRequested = false;
    let extRequested = new Set<string>();
    let fileRequested = new Set<string>();
    files.forEach(file => {
        if(!isFolderIconRequested && file.file_type === 'dir' && cache.folderIcon == null ){
            req.push(file.full_path);
            reqTypeByFile[file.full_path] = 'dir';
            isFolderIconRequested = true;
            return;
        }

        if (file.file_type === 'dir') return;

        const fileNameSplitByDot = file.name.split('.');
        if (fileNameSplitByDot.length <= 1) return;
        const ext = fileNameSplitByDot.pop();
        if(ext === "exe" && !fileRequested.has(file.full_path) && !cache.file[file.full_path]){
          req.push(file.full_path);
          reqTypeByFile[file.full_path] = 'file';
          fileRequested.add(file.full_path);
          return;
        }

        if(ext != undefined && !extRequested.has(ext) && ext && !cache.ext[ext]){
            req.push(file.full_path);
            reqTypeByFile[file.full_path] = 'ext';
            extRequested.add(ext);
          return;
        }

    });
    return {req, reqTypeByFile};
}

export const mergeIconCacheFromUpdateRes = (cache:IconCache, reqTypeByFile:TypeByFile, icons:{[index:string]:string}) => {
    for(let [filePath, iconBinary] of Object.entries(icons)){
        if(reqTypeByFile[filePath] === 'dir'){
            cache.folderIcon = iconBinary;
        }else if(reqTypeByFile[filePath] === 'ext'){
            const ext = filePath.split('.').pop();
            if(ext){
                cache.ext[ext] = iconBinary;
            }
        }else if(reqTypeByFile[filePath] === 'file'){
            cache.file[filePath] = iconBinary;
        }
    }
    return cache;
}

export const getIconBinary = (cache:IconCache, file:FileItem) => {
    if(file.file_type === 'dir'){
        return cache.folderIcon;
    }

    const fileNameSplitByDot = file.name.split('.');
    if (fileNameSplitByDot.length <= 1) return "";
    const ext = fileNameSplitByDot.pop();
    if(ext === "exe" && cache.file[file.full_path]){
      return cache.file[file.full_path];
    }

    if(ext && cache.ext[ext]){
        return cache.ext[ext];
    }
    return null;
}

