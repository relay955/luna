English (Translated By AI)
# Luna - A Rust-based ordinary file explorer
## Features
- File exploration

## Planned Features
- Current directory tree view (like in an IDE)
- Screen splitting system (2-split, 4-split)
- Picture-in-Picture (PIP) screen splitting system
- Bulk renaming based on rules
- Built-in preview (video, photo, music, text editor, HEX viewer)
- Integrated browser
- Favorites
    - Search-command-based favorites
- Directory size analysis (like TreeSize)
- Tabbed browsing
- Support for protocols like SAMBA/FTP
- Encryption system (encrypted folders, encrypted files, etc.)
- Hashing and fast search (like Everything, though indexing speed may be slower since it doesn’t use NTFS Master File Table)
- File tagging / tag-based search
- GPT-based search
- Flattening nested directories
- Online synchronization (like rsync)
- Online file sharing (generates temporary URLs with an expiration date; port forwarding must be done manually)

## Encryption System (Planned)
### Implementation Goals
- File-level encryption (folder-level recognition)
- Separate encryption for each file's metadata, allowing immediate metadata access (like filenames) without needing to decrypt the entire file
- Use of the `.encrypted` extension

### Mechanism
- All encrypted file names should be randomized 40-character hexadecimal strings.
- A header file will exist in the marked folder location, with the header file always starting with a 4-digit hexadecimal representation of the password hash. When creating other files, if this 4-digit representation appears in the name, the name will be changed to avoid conflict.
- The meta header file will be encrypted with the same key and will store the full path of the `.encrypted` file and the actual file metadata (such as the real name of folders, filenames, extensions, icons, a brief image, and a folder profile if there are pictures in the folder).
- When Luna detects a `.encrypted` file, it will recognize the folder as encrypted and first attempt to decrypt the metadata.
- When attempting to open a file in Luna, the file will automatically be decrypted and saved to a temporary folder (RAM disk recommended), and then executed with the appropriate program.
- To support multi-image interpretation, Luna will offer a temporary decryption feature for the entire folder.
___
한국어
# Luna - 러스트 기반의 평범한 파일탐색기
luna는 Rust, Tauri 기반으로 구현된 Windows용의 평범한 파일 탐색기입니다.

## 기능
- 파일 탐색

## 기능(구현예정)
- 현재 디렉터리 트리뷰(like IDE)
- 화면 분할 시스템(2분할, 4분할)
- PIP 화면 분할 시스템
- 규칙에 따른 일괄 이름 변경
- 내장형 미리보기(동영상, 사진, 음악, 텍스트편집기, HEX뷰어)
- 내장형 브라우저
- 즐겨찾기
  - 검색명령 기반 즐겨찾기
- 디렉터리 용량분석 (like treesize)
- 탭 브라우징
- SAMBA/FTP 등 프로토콜 지원
- 암호화 시스템 (인크립션 폴더, 인크립션 파일 등)
- 해시 및 고속검색 (like everything, 단 NTFS 마스터파일테이블을 사용하지 않으므로 인덱싱 속도는 느릴수있음)
- 파일 태깅 / 태그기반 검색
- GPT 기반 검색
- 파일 네스팅 제거
- 온라인 동기화 (like rsync)
- 온라인 파일공유 (임시 URL이 생성되고 만료일자 지정가능, 포트포워딩은 직접 해야함)

## 암호화 시스템
### 구현 목표
- 파일 단위 암호화 (폴더레벨 인식)
- 각 파일에 대한 메타데이터 별도암호화. 파일을 바로 해독하지 않고도 즉각적인 메타데이터 확인(파일이름 등)
- .encrypted 확장자 사용

### 메커니즘
- 암호화된 모든 파일 이름은 랜덤한 16진수 이름(40자)를 갖도록 함
- 폴더 마킹한 위치는 헤더 파일이 존재하며, 헤더파일은 항상 시작값이 (비밀번호해시 4자리 16진수)로 시작. 다른파일 생성시에는 (비밀번호 해시 4자리 16진수)로 시작할 수 없도록 이 명칭이 붙어있으면 다른이름으로 변경
- 메타헤더파일은 같은 키로 암호화되어있고, 프로그램으로 로딩할 수 있는 .encrypted 파일의 풀경로와 실제 파일 메타데이터(폴더라면 폴더의 실제이름, 파일이름, 확장자, 아이콘, 간단한 사진, 폴더에 사진이 있는경우 폴더 프로파일) 정보가 저장되어있음.
- luna는 .encrypted 파일을 확인하면 해당 폴더가 암호화되어있음으로 인지하고 먼저 메타데이터 해독 시도
- luna에서 파일을 열려고 시도하면, 해당 파일이 자동으로 decrypt되어 임시 폴더(램디스크를 추천)에 저장되고, 적절한 프로그램으로 실행
- 다중사진 해석을 지원하기 위해 폴더 전체 일시 디크립션 기능 지원

### (주의) 콜드부트 공격, 부채널 공격 취약성
luna의 암호화 시스템은 AES256을 이용합니다.
하지만, 메모리를 스캔하는 콜드부트공격이나 부채널공격에 대한 방어 기능을 제공하지 않습니다.
또한 luna의 임시 복호화된 파일은 임시폴더에 저장되었다가 삭제되는 방식을 사용하므로, 반드시 램디스크에 저장하는것이 추천됩니다.
콜드부트 공격을 보호하기 위해 다음 수칙을 지켜주세요.

- 임시 복호화 폴더는 반드시 램디스크로 지정합니다.
- 보호모드 진입 후, 작업이 종료되면 반드시 보호모드를 해제합니다.
- 보호모드 해제 이후에도 메모리에 남아있을 수 있는 복호화된 파일을 완전히 메모리에서 소거하기 위해, 컴퓨터를 종료하고 일정시간 전원을 차단한 후 다시 부팅합니다.

### (주의) 짧은 비밀번호 또는 사전대입공격에 대한 취약성
파일을 지키고싶다면, 암호는 반드시 12자 이상의 특수문자를 포함하여 입력해주세요. 무차별 대입 공격에 취약할 수 있습니다.

### (주의) 파일 손실에 대한 대응
luna는 파일 손실에 대한 대응책을 제공하지 않습니다. 파일을 암호화했더라도 반드시 백업본을 마련해두세요.

### (주의) 파일 접근 기록으로부터 보호
luna는 파일에 접근한 기록을 제거하는 기능을 제공하지 않습니다. 파일에 접근한 기록 마저 남기지 않아야 한다면, 다음 수칙을 지켜주세요.
- 파일명을 암호화한 상태로 복호화하는 기능을 켜두세요. 파일명을 통해 흔적을 찾는 것을 방지할 수 있습니다.
- luna의 자체 제공 파일 뷰어를 사용합니다. 메모리 내에서 복호화를 수행하므로 파일에 접근한 기록이 남지 않습니다.
- 보안 작업용 별도의 OS를 준비하고 bitlocker 등으로 시스템 전체를 암호화합니다. 이 때, bitlocker 암호 해제는 사용자가 부팅시마다 키를 입력하는 방법으로 설정합니다.
그리고 작업이 필요한 순간에만 사용합니다.
