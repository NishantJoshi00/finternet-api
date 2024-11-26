## Get File Descriptor (Local Resources)
```mermaid
sequenceDiagram
    participant User Process
    participant System Call Interface
    participant Process FD Table
    participant VFS
    participant File System
    participant Inode Cache
    participant Device Driver
    
    User Process->>System Call Interface: open("/path/to/file", flags, mode)
    activate System Call Interface
    
    System Call Interface->>VFS: path_lookup("/path/to/file")
    activate VFS
    
    VFS->>File System: lookup path components
    activate File System
    
    File System->>Inode Cache: get_inode()
    activate Inode Cache
    
    alt Inode in cache
        Inode Cache-->>File System: return cached inode
    else Inode not in cache
        Inode Cache->>Device Driver: read inode from disk
        Device Driver-->>Inode Cache: return inode data
        Inode Cache-->>File System: return new inode
    end
    deactivate Inode Cache
    
    File System-->>VFS: return dentry + inode
    deactivate File System
    
    VFS->>VFS: create file structure
    Note over VFS: Initialize file object<br/>Set file operations<br/>Set read/write position
    
    VFS->>Process FD Table: get_unused_fd()
    activate Process FD Table
    
    Process FD Table->>Process FD Table: find lowest available FD
    Process FD Table-->>VFS: return new fd
    
    VFS->>Process FD Table: fd_install(fd, file)
    Note over Process FD Table: Link FD to file structure
    deactivate Process FD Table
    
    VFS-->>System Call Interface: return file structure
    deactivate VFS
    
    System Call Interface-->>User Process: return fd
    deactivate System Call Interface
    
    Note over User Process: File descriptor can now<br/>be used with read(),<br/>write(), etc.
```

## Open Life Cycle (Remote Resources)
```mermaid


sequenceDiagram
    participant User Process
    participant VFS
    participant FUSE Kernel
    participant FUSE Daemon
    participant SSHFS Process
    participant SSH Client
    participant Remote SSHD
    participant Remote FS
    
    User Process->>VFS: open("/mnt/remote/file", flags)
    activate VFS
    
    VFS->>FUSE Kernel: lookup operation
    activate FUSE Kernel
    
    FUSE Kernel->>FUSE Daemon: FUSE_LOOKUP
    activate FUSE Daemon
    
    FUSE Daemon->>SSHFS Process: lookup path
    activate SSHFS Process
    
    SSHFS Process->>SSH Client: sftp-open
    activate SSH Client
    
    SSH Client->>Remote SSHD: encrypted SFTP request
    activate Remote SSHD
    
    Remote SSHD->>Remote FS: open file
    activate Remote FS
    
    Remote FS-->>Remote SSHD: file handle
    deactivate Remote FS
    
    Remote SSHD-->>SSH Client: encrypted SFTP response
    deactivate Remote SSHD
    
    SSH Client-->>SSHFS Process: file handle
    deactivate SSH Client
    
    SSHFS Process->>SSHFS Process: create local handle mapping
    SSHFS Process-->>FUSE Daemon: status + handle
    deactivate SSHFS Process
    
    FUSE Daemon-->>FUSE Kernel: file info
    deactivate FUSE Daemon
    
    FUSE Kernel->>FUSE Kernel: create kernel file handle
    FUSE Kernel-->>VFS: file handle
    deactivate FUSE Kernel
    
    VFS-->>User Process: file descriptor
    deactivate VFS

    Note over User Process,Remote FS: Subsequent read operations will follow a similar path


```










### Read Life Cycle
```mermaid
sequenceDiagram
    participant User Process
    participant System Call Interface
    participant VFS
    participant File System
    participant Device Driver
    participant Hardware

    User Process->>System Call Interface: read(fd, buffer, count)
    System Call Interface->>VFS: sys_read()
    Note over VFS: Validate file descriptor<br/>Check permissions

    VFS->>File System: file->f_op->read()
    Note over File System: Convert to filesystem<br/>specific operations

    alt Regular File
        File System->>Device Driver: block device operations
        Device Driver->>Hardware: Read from disk
        Hardware->>Device Driver: Return data
        Device Driver->>File System: Return data
    else Character Device
        File System->>Device Driver: character device operations
        Device Driver->>Hardware: Read from device
        Hardware->>Device Driver: Return data
        Device Driver->>File System: Return data
    end

    File System->>VFS: Return data
    VFS->>System Call Interface: Copy to user space
    System Call Interface->>User Process: Return bytes read
```











---
### Knowledge

How multiple file descriptors can exist at the same time?
```mermaid
sequenceDiagram
    participant Process A
    participant Process B
    participant VFS
    participant File System
    participant Lock Manager
    
    Process A->>VFS: open("/path/file", O_RDWR)
    VFS->>File System: get_inode()
    File System-->>VFS: inode
    VFS->>VFS: create file structure
    VFS-->>Process A: fd1
    
    Process B->>VFS: open("/path/file", O_RDWR)
    VFS->>File System: get_inode()
    Note over File System: Same inode returned
    File System-->>VFS: inode
    VFS->>VFS: create new file structure
    VFS-->>Process B: fd2
    
    Note over Process A,Process B: Both processes can now read/write
    
    Process A->>VFS: fcntl(fd1, F_SETLK, ...)
    VFS->>Lock Manager: try_lock(inode, range)
    Lock Manager-->>VFS: lock acquired
    VFS-->>Process A: success
    
    Process B->>VFS: write(fd2, ...)
    VFS->>Lock Manager: check_lock(inode, range)
    Lock Manager-->>VFS: locked by Process A
    VFS-->>Process B: EAGAIN
```


```mermaid
stateDiagram-v2
    [*] --> Inode
    
    state Inode {
        [*] --> FileSystem
        state FileSystem {
            Metadata
            DataBlocks
        }
        
        state "File Table" as FT {
            state "File Structure A" as FSA
            state "File Structure B" as FSB
        }
        
        state "Process FD Tables" as PFD {
            state "Process A FDs" as PA
            state "Process B FDs" as PB
        }
    }
    
    Metadata --> FSA
    Metadata --> FSB
    FSA --> PA
    FSB --> PB
```


### Notes

Type of Locks
- Advisory Locks (flock)
- Mandatory Locks
- Read/Write Locks

- The kernel doesn't enforce locking, it's something that is done by the applications explicitly
- The access remote file there is FUSE deamon

- File Descriptor -> File Handle -> (inode, filesystem) -> driver specific syscalls.

```mermaid
graph TD
	K[Kernel]
	SD[SystemD]
	VFS
	DD[Device Driver]
	D[Disk]
	subgraph Process
		FD[File Descriptor]
	end

	Process -->|syscalls| K
	SD --> Process
	K --> SD
	K --> VFS
	VFS --> DD
	DD --> D
```


