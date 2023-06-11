# gitignore

```
gitignore [<options>] project

download gitignore from https://github.com/github/gitignore
https://raw.githubusercontent.com/github/gitignore/main/[Project].gitignore

options
    -a --append path에 파일이 있을 경우 append
    -r --replace path에 파일이 있을 경우 replace
    -p --path <path> 지정한 경로에 .gitignore 생성, 없을 경우 현재 directory
```

## cli program tutorial

https://betterprogramming.pub/building-cli-apps-in-rust-what-you-should-consider-99cdcc67710c

## ignore files

파일목록: https://api.github.com/repos/github/gitignore/git/trees/main?recursive=0
파일정보: https://api.github.com/repos/github/gitignore/git/blobs/c6bba591381216b569cdcb512d98b53c53fd167d
base64에 newline을 없애고 decode하면 내용가져올 수 있음.

## temp 파일들 어디 저장하면 좋을까?

...

## 추가로 있으면 좋은 기능

- suggestion
  - https://xo.dev/articles/hacking-command-recommendation
