\pagebreak

# Main Workflow

## Overview

Our main workflow consists of the following steps:

| #   | Step Title                           |
| --- | ------------------------------------ |
| 1   | Set Disk Layout                      |
| 2   | Format Partitions                    |
| 3   | Mount Filesystems                    |
| 4   | Select Mirror Server                 |
| 5   | Select Profile                       |
| 6   | Download Stage3 Tarball              |
| 7   | Decompress Stage3 Tarball            |
| 8   | Update System Packages               |
| 9   | Miscellaneous Startup Configurations |
| 10  | Miscellaneous Usage Configurations   |



## Step 1: Set Disk Layout

### Selection of Partitions

DeployKit shall ask the user to select the boot mode (EFI or BIOS).

If the user selects EFI:
This program shall ask the user to select the partitions ESP and ROOT, from the given list of partitions.

If the user selects BIOS:
This program shall ask the user to select the partition ROOT, from the given list of partitions.

### Go for Management

If the user claims that they need to manage partitions,
this program shall spawn an interactive shell,
where the user may use CLI/TUI tools like `cfdisk` and `parted` to manage the disk layout.
When the interactive shell exits, the user shall be asked the same question,
with the new partitions available in the list.

### Partition Size Warning

If the user selects any partition, this program shall determine whether the partition is big enough according to the following standard:

| Partition | Size Threshold |
| --------- | -------------- |
| ROOT      | 25 GiB         |
| ESP       | 200 MiB        |

- ESP: 200 MiB
- ROOT: 25 GiB

If the partition is not big enough, this program shall inform the user with a dialogue,
requiring an extra confirmation.

### Other Details

According to the choices made by the user, this program shall set partition flags (i.e. `boot` and `esp`) properly.


## Step 2: Format Partitions

If any of the partitions, designated from the previous step, is mounted,
it shall be unmounted along with any possibly existing mounted subdirectories inside it.

DeployKit shall format these partitions. The ESP shall be formatted as VFAT, and the ROOT shall be formatted as Ext4.

## Step 3: Mount Filesystems

DeployKit shall mount the formatted filesystems at proper locations:

| Partition | Mount Point         |
| --------- | ------------------- |
| ROOT      | `/tmp/.dk/aosc`     |
| ESP       | `/tmp/.dk/aosc/efi` |

## Step 4: Select Mirror Server

DeployKit asks the user to select a mirror server.
The URL for the list of mirror servers is hardcoded in this program when building.

The list contains certain information for each mirror server, e.g. title, location, and sponsor.

## Step 5: Select Profile

DeployKit asks the user to select a profile from the list, which was fetched from the mirror server as selected in the previous step.

A profile is a classifier of a stage3 tarball,
which corresponds to the stage3 tarball filename minus the versioning information (e.g. date of building).

A profile should look like "`aosc-os_gnome+nvidia_amd64`" (or a human-readable version "`GNOME (with NVIDIA) (arm64)`"),
which corresponds to the stage3 tarball named "`aosc-os_gnome+nvidia_20220122_amd64.tar.xz`".

It will be nice if the list can be organized hierarchically,
as a two-layer `ARCH/DE` tree, where the current architecture may be highlighted (e.g. "`amd64 (current device)`").

## Step 6: Download Stage3 Tarball

DeployKit downloads the stage3 tarball from the server, showing a progress bar to the user.
Along with the progress bar, some details should be displayed:

- Current Speed
- Total Size
- Downloaded Size
- Downloaded Percentage
- Prediction for Remaining Time

## Step 7: Decompress Stage3 Tarball

This step should be as simple as:

```
cd $DKROOT
tar -xpvf $TARBALL
```

## Step 8: Update System Packages

This step should be as simple as:

```
apt update
apt full-upgrade -y
```

## Step 9: Miscellaneous Startup Configurations

### grub-install

According to the boot mode as selected in Step 1, DeployKit shall install the bootloader differently.

### grub-mkconfig

// TODO

## Step 10: Miscellaneous Usage Configurations

// TODO

### Add User
### Set Password
### Set Hostname
### Set Language
### Set Timezone



\pagebreak

# Error Handling

// TODO

## Multiple Instances

DeployKit cannot run multiple instances simultaneously.
This program may use a file lock to prohibit multitasking.

## Exit and Resume

DeployKit may exit unexpectedly in the middle of any step in the main workflow.
