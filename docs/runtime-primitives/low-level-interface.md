In a Linux operating system, when working with files following are the key components that take part in the process:

1. Linux Kernel (kernel)
2. System Daemon (systemd)
3. Virtual File System (VFS)
4. Device Drivers (DD)
5. Hardware (Disk)
6. Process (P)
7. File Descriptor (FD)
8. FUSE Daemon (FUSE)

How does the interaction looks like?

- The user creates a new process by requesting system daemon. SystemD asks kernel to spawn a new process.
- The process requests the kernel to give access to a file.
  - the process makes a syscall to the kernel
  - the kernel verifies the user land process and checks if the file is accessible by the process
  - the VFS asks the filesystem, for inode information (this is fetched from the DD)
    - There's a catch here, if the its' a remote resource, the VFS asks the FUSE daemon, when then relays the request to the FS handler process when they does the inode lookup.
  - the kernel return the file descriptor by combining the inode, dentry and other information
- Now that we have a file descriptor, we can ask to do operations on it.
- When performing the operation the VFS validates the FD and it's permissions.
- The VFS then asks the filesystem to perform the operation.
- the filesystem converts it to block device operations and asks the DD to perform the operation.
- The DD then performs the operation on the hardware.
- and returns the result back to the filesystem.
- the filesystem then returns the result back to the VFS.
- and the VFS moves the retrieved data to the user land process.

Now we are designing a new system, which is quite similar to the linux operating system. In our system, we are trying to store, read, write and work with assets. Here we can draw a parallel that an asset on this system is similar to a file on the linux os.

If we are saying that an asset is similar to a file, then what are the components that we need to design in our system?

- system daemon is like the interface via which the user will interact with this system.
  in our case, it's the API.
- kernel the core of the system, which overlooks the operations and manages the resources.
  in our case, it's the core of the system.
- VFS is responsible for abstracting the filesystem APIs into a common interface.
  We can consider this to be a system that provide virtualization over the entire finternet resource management.
- Device Drivers + FileSystem constitute to implement the operations on the hardware.
  In our case, it's the storage system.
- Hardware is the storage system.
  In our case, it could be a ledger or a database.
- Process is any operation that the user wants to perform.
  it could either be a direct request from the user, a cron that is scheduled to run or a event hook
- File Descriptor is the instance of the asset that the process holds from processing. This is an active reference in response to a `open` syscall. Which in our case would be an intent to perform an action. A file descriptor in our case would look more like a purpose bound reference to the asset.

Actions (Syscalls)

```c

enum IntentPurpose {
    O_DEBIT = 1,
    O_CREDIT = 2,
    O_CREATE = 4,
    O_TRUNCATE = 8,
    O_READ = 16,
};

```

- `intend`
  ```c
  intent_d intend(void* asset_id, uint32_t intent); // intent is a bitmask of the operations that the user wants to perform
  ```
- `done` this is similar to the `close` syscall in linux
  `c
    int* done(intent_d intent); // it can error out
    `

  > `debit`
  >
  > ```c
  > int* debit(intent_d intent, uint32_t units); // it can error out
  > ```
  >
  > `credit`
  >
  > ```c
  > int* credit(intent_d intent, uint32_t units); // it can error out
  > ```

- `transfer`
  ```c
  int* transfer(intent_d from, intent_d to, uint32_t units); // it can error out
  ```
- `view`
  ```c
  int* view(intent_d intent); // it can error out
  ```
- `execute`
  ```c
  int* execute();
  ```
- `load_module` this can be associated with adding support for a new asset on the system (or onboarding a remote asset)

  ```c

  ```

- `unload_module` this can be associated with deleting an asset from the system (or offboarding a remote asset)
- `mod_resolv` Resolve the module to be used for performing fundamental operations on a specific asset.
- `mount` Mount a local/remote asset management system to finternet
- `unmount` Unmount a local/remote asset management system from finternet
- `useradd` Add a new user to the system
- `usermod` Modify an existing user
- `userdel` Delete a user from the system
- `chmod` Change the permissions of an asset
- ``
