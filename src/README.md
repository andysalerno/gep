# 'Good Enough Passwords'

## Logic

### Examples

The following invocations of `gep` are shown with the hashing logic they will utilize:

* `gep --site facebook --username zuck`
    * hash('{master_pass}' + 'facebook' + 'zuck')
    * real output when 'master_pass' is 'password' and dictionary is the default dict: `fillmein`
* `gep -s facebook -u zuck --num 123`
    * hash('{master_pass}' + 'facebook' + 'zuck' + '123')
    * real output when 'master_pass' is 'password' and dictionary is the default dict: `fillmein`

`gep` passwords will always include a number at the end, which is generated by one of three methods:

### Salt values, numbers, and special characters

`gep` can optionally salt the precursor with either a random `u8` or a user-specified `u8`.  This option is not intended to increase the strength of the password (since it doesn't add much entropy); instead it exists simply to allow generating multiple passwords from the same (site/username/masterpass) values.

Because so many sites require passwords to contain numbers and special characters, `gep` passwords always end in a delim character followed by a number. The number is the salt number (if a salt is to be used), or else derived from the hash (if no salt is to be used). The delim character is `':'` (no option exists to change this, but the value can be modified in the source).

More formally:

1. If the flag `--rand_num` (or `-r`) is present, a random `u8` is generated using `OsRng`, appended to the hash precursor, and appended to the final password output.
1. If the user argument `--num` (or `-n`) is present, its value is used (as a `u8`), and just like `-r` it is appended to the hash precursor and appended to the final password output. 
1. If neither the `--num` nor `--rand_num` options are present, no number is appended to the hash precursor, and the final byte of the hash output is interpreted as a `u8` and appended to the final password output.
