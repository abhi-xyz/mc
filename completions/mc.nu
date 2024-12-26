module completions {

  # Copies files or directories with options for recursion and overwriting.
  export extern mc [
    ...source: string         # Source file or directory to copy
    destination: string       # Destination file or directory
    --force(-f)               # Overwrite destination if it exists
    --hard-link               # Hard link file
    --symlink                 # Symbol link file
    --reflink                 # Ref link file Similar to hardlink except modify one doesn't affect the other
    --verify                  # Verify hash of folder / file once copied
    --no-progress             # Disable progress bar
    --no-keep-awake           # Disable keep system awake while copy
    --keep-display-awake      # Keep display awake while copy
    --help(-h)                # Print help
  ]

}

export use completions *
