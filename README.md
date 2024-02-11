# C Git (`Custom Git`)

Git command mapper.

## Usage

```bash
cg help
```

<br/>

### Initilization
######  Initilize repo.
```bash
cg init
```
######  Uninitilize repo.
```bash
cg uninit
```

<br/>

### Commit
######  Commit with message
```bash
cg commit -m "MESSAGE"
```
######  Commit random message
```bash
cg commit -r
```

<br/>

### Add
######  Add files
```bash
cg add .
# or
cg add FILE-A FILE-B
```

<br/>

### Origin
######  Get Origin address
```bash
cg origin
```
######  Add origin
```bash
cg origin https://github.com/mdmahikaishar/custom-git.git
```

<br/>

### Branch.
######  Get current branch name.
```bash
cg branch
```
######  Go to branch.
```bash
cg branch main
```
######  Create new branch.
```bash
cg branch -c hello
```
######  Branch list
```bash
cg branch -l

```

<br/>

### Push
######  Push upstream.
```bash
cg push origin main
```

<br/>

### Status
```bash
cg status
```

<br/>

### Log
```bash
cg log
```

<br/>

### Merge
```bash
cg merge
```